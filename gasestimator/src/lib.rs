use forest_json::message::json::MessageJson;
use forest_rpc_api::gas_api;
use fvm_shared::{econ::TokenAmount, message::Message};
use rpc::{RpcEndpoint, RpcError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GasEstimatorError {
    #[error("rpc request error: {0}")]
    RpcRequestError(#[from] RpcError),
}

pub async fn estimate_msg_gas(rpc: RpcEndpoint, msg: Message) -> Result<Message, GasEstimatorError> {
    let msg_json = MessageJson::from(msg.clone());

    let max_fee = TokenAmount::from_nano(1_000_000_000);

    match rpc
        .post::<_, MessageJson>(gas_api::GAS_ESTIMATE_MESSAGE_GAS, vec![
            json!(msg_json),
            json!({"MaxFee": max_fee.atto().to_string(),}),
            json!([]),
        ])
        .await
    {
        Ok(res) => {
            let MessageJson(res) = res;
            Ok(res)
        }
        Err(err) => Err(GasEstimatorError::RpcRequestError(err)),
    }
}
