use url::{Url, ParseError};
use jsonrpc_v2::RequestObject;
use reqwest::{
    Client,
    header::{CONTENT_TYPE, AUTHORIZATION},
};
use std::{
    str::FromStr,
    sync::{
        Arc,
        atomic::{
            AtomicUsize,
            Ordering::{SeqCst, Relaxed},
        },
    },
    time::Duration,
};
use log::{error, info};
use serde_json::json;
use thiserror::Error;

const RPC_START_ID: usize = 1000;

#[derive(Clone)]
pub struct RpcEndpoint {
    url: Url,
    request_id: Arc<AtomicUsize>,
    bearer_token: String,
    debug: bool,
}

impl FromStr for RpcEndpoint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;

        Ok(Self {
            url: url,
            request_id: Arc::new(AtomicUsize::new(RPC_START_ID)),
            bearer_token: String::default(),
            debug: false,
        })
    }
}

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("low level error: {0}")]
    LowLevelError(#[from] reqwest::Error),
    #[error("fail request")]
    RequestError,
    #[error("rpc application error {0}")]
    RpcApplicationError(serde_json::Value),
    #[error("rpc response parse error")]
    RpcResponseParseError,
    #[error("rpc application result parse error: {0}")]
    RpcApplicationResultParseError(#[from] serde_json::Error),
    #[error("unknown error")]
    Unknown,
}

impl RpcEndpoint {
    pub fn new(url: String, bearer_token: String) -> Result<Self, ParseError> {
        let url = Url::parse(url.as_str())?;

        Ok(Self {
            url: url,
            request_id: Arc::new(AtomicUsize::new(RPC_START_ID)),
            bearer_token: bearer_token.to_string(),
            debug: false,
        })
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub async fn post<
        T1: serde::Serialize,
        T2: for<'de>serde::Deserialize<'de>,
    >(&self, method: &str, params: T1) -> Result<T2, RpcError> {
        let req = RequestObject::request()
            .with_params(json!(params))
            .with_method(method)
            .with_id(self.request_id.load(Relaxed) as i64)
            .finish();

        if self.debug {
            info!("Request: {:?}", req);
        }

        self.request_id.fetch_add(1, SeqCst);

        let cli = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        let res = cli
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
            .json(&req)
            .send()
            .await?;

        let resp;

        match res.error_for_status() {
            Ok(res) => {
                info!("POST -> {} SUCCESS", method);
                resp = res;
            },
            Err(err) => {
                error!("POST -> {} - {} FAIL", method, err);
                return Err(RpcError::LowLevelError(err));
            },
        }

        let res = resp;

        let res = res.json::<serde_json::Value>().await?;
        if self.debug {
            info!("Response: {}", res);
        }

        if res.get("result").is_some() {
            match serde_json::from_value::<T2>(res.get("result").unwrap().clone()) {
                Ok(res) => return Ok(res),
                Err(err) => return Err(RpcError::RpcApplicationResultParseError(err)),
            };
        }
        if res.get("error").is_some() {
            return Err(RpcError::RpcApplicationError(res.get("error").unwrap().clone()));
        }
        Err(RpcError::Unknown)
    }
}
