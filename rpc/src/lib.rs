use url::{Url, ParseError};
use jsonrpc_v2::RequestObject;
use reqwest::{
    blocking::Client,
    header::{CONTENT_TYPE, AUTHORIZATION},
    Error,
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
        T1: fvm_ipld_encoding::ser::Serialize,
        T2: for<'de> fvm_ipld_encoding::de::Deserialize<'de>,
    >(&self, method: &str, params: T1) -> Result<T2, Error> {
        let req = RequestObject::request()
            .with_params(json!(params))
            .with_method(method)
            .with_id(self.request_id.load(Relaxed) as i64)
            .finish();

        self.request_id.fetch_add(1, SeqCst);

        let cli = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        let res = cli
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
            .json(&req)
            .send()?;

        match res.error_for_status() {
            Ok(res) => {
                info!("POST -> {} SUCCESS", method);
                res.json::<T2>()
            },
            Err(err) => {
                error!("POST -> {} - {} FAIL", method, err.status().unwrap());
                Err(err)
            },
        }
    }
}
