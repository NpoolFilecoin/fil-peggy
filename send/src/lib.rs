use forest_key_management::{
    KeyInfo,
};
use fvm_shared::{
    address::Address,
    econ::TokenAmount,
};
use rpc::RpcEndpoint;
use thiserror::Error;
use forest_json::{
    cid::CidJson,
};
use mpool::{mpool_push, MpoolError};
use state::{wait_msg, StateError};

#[derive(Error, Debug)]
pub enum SendError {
    #[error("mpool call error")]
    MpoolCallError(#[from] MpoolError),
    #[error("state call error")]
    StateCallError(#[from] StateError),
}

pub async fn send(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    to: Address,
    value: TokenAmount,
) -> Result<CidJson, SendError> {
    match mpool_push::<_, CidJson>(rpc.clone(), from, from_key_info, to, 0, value, Vec::<CidJson>::new()).await {
        Ok(res) => {
            let _ = wait_msg::<serde_json::Value>(rpc, res.clone()).await?;
            Ok(res)
        },
        Err(err) => Err(SendError::MpoolCallError(err)),
    }
}
