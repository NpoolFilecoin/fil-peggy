use url::Url;
use std::sync::atomic::Ordering::{SeqCst, Relaxed};
use jsonrpc_v2::RequestObject;
use reqwest::{
    Client,
    blocking::Client,
    header::{CONTENT_TYPE, AUTHORIZATION},
};
use std::{
    str::FromStr,
};
use anyhow::Error;

const RPC_START_ID: usize = 1000;

#[derive(Try)]
struct RpcEndpoint {
    url: Url,
    request_id: Arc<AtomicUsize>,
    cli: Client,
    bearer_token: String,
    non_block: bool,
}

impl FromStr for RpcEndpoint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cli = blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(600))
            .build()?;

        let url = Url::parse(s)?;

        Ok(Self {
            url: url,
            request_id: RPC_START_ID,
            cli: cli,
            bearer_token: String::default(),
            non_block: false,
        })
    }
}

impl RpcEndpoint {
    fn new(url: &str, bearer_token: &str, non_block: bool) -> Result<Self, &str> {
        let cli = match non_block {
            true => cli = Client::builder().build()?;
            false => cli = blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(600))
                .build()?;
        }

        let url = Url::parse(url)?;

        Ok(Self {
            url: url,
            request_id: RPC_START_ID,
            cli: cli,
            bearer_token: bearer_token.to_string(),
            non_block: non_block,
        })
    }

    async fn post<T1, T2>(&self, method: &str, params: T1) -> Result<T2, &str> {
        let req = RequestObject::request()
            .with_params(json!(params))
            .with_method(method)
            .with_id(self.request_id.load(Relaxed))
            .finish();

        self.request_id.fetch_add(1, SeqCst);

        let res = self.cli
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
            .json(&req)
            .send()
            .await?;
        if !res.status().is_success() {
            error!("POST -> {} - {} FAIL", method, res.status());
            return Err("request fail");
        }

        info!("POST -> {} SUCCESS", method);

        res.json()
    }
}
