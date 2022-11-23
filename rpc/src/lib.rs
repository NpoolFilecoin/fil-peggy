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

const RPC_START_ID: usize = 1000;

#[derive(Clone)]
pub struct RpcEndpoint {
    url: Url,
    request_id: Arc<AtomicUsize>,
    bearer_token: String,
}

impl FromStr for RpcEndpoint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;

        Ok(Self {
            url: url,
            request_id: Arc::new(AtomicUsize::new(RPC_START_ID)),
            bearer_token: String::default(),
        })
    }
}

impl RpcEndpoint {
    pub fn new(url: String, bearer_token: String) -> Result<Self, ParseError> {
        let url = Url::parse(url.as_str())?;

        Ok(Self {
            url: url,
            request_id: Arc::new(AtomicUsize::new(RPC_START_ID)),
            bearer_token: bearer_token.to_string(),
        })
    }

    pub async fn post<
        T1: serde::Serialize,
        T2: for<'de>serde::Deserialize<'de>,
    >(&self, method: &str, params: T1) -> Result<T2, String> {
        let req = RequestObject::request()
            .with_params(json!(params))
            .with_method(method)
            .with_id(self.request_id.load(Relaxed) as i64)
            .finish();

        self.request_id.fetch_add(1, SeqCst);

        let cli = Client::builder()
            .timeout(Duration::from_secs(600))
            .build();
        if cli.is_err() {
            return Err(String::from(format!("fail build client: {:?}", cli)));
        }

        let cli = cli.unwrap();
        let res = cli
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
            .json(&req)
            .send()
            .await;
        if res.is_err() {
            return Err(String::from(format!("fail request: {:?}", res)));
        }

        let res = res.unwrap();
        let resp;

        match res.error_for_status() {
            Ok(res) => {
                info!("POST -> {} SUCCESS", method);
                resp = res;
            },
            Err(err) => {
                error!("POST -> {} - {} FAIL", method, err);
                return Err(String::from(format!("fail request: {} {}", method, err)));
            },
        }

        let res = resp;

        let res = res.json::<serde_json::Value>().await;
        if res.is_err() {
            return Err(String::from(format!("fail parse res: {:?}", res)));
        }

        let res = res.unwrap();
        if res.get("result").is_some() {
            match serde_json::from_value::<T2>(res.get("result").unwrap().clone()) {
                Ok(res) => return Ok(res),
                Err(err) => return Err(String::from(format!("fail parse result: {}", err))),
            };
        }
        if res.get("error").is_some() {
            return Err(String::from(format!("{:?}", res.get("error").unwrap())));
        }
        Err(String::from(format!("unknow error: {:?}", res)))
    }
}
