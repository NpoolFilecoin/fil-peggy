use rpc::{RpcEndpoint, RpcError};
use forest_json::{
    cid::CidJson,
};
use forest_rpc_api::{
    state_api,
};
use serde::{Deserialize, Serialize};
use forest_ipld::json::IpldJson;
use forest_blocks::{
    tipset_keys_json::TipsetKeysJson,
};
use serde_json::json;
use std::fmt;
use fvm_shared::{
    error::ExitCode,
};
use thiserror::Error;
use std::{str::FromStr, fmt::Debug};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiptJson {
    exit_code: ExitCode,
    #[serde(rename = "Return")]
    return_data: Option<String>,
    gas_used: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageLookup {
    pub receipt: ReceiptJson,
    #[serde(rename = "TipSet")]
    pub tipset: TipsetKeysJson,
    pub height: i64,
    pub message: CidJson,
    pub return_dec: Option<IpldJson>,
}

impl fmt::Debug for MessageLookup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Receipt {}", serde_json::to_string(&self.receipt).unwrap())?;
        write!(f, "Tipset {:?}", self.tipset)?;
        write!(f, "Height {}", self.height)?;
        write!(f, "Message {:?}", self.message)?;
        write!(f, "ReturnDec {:?}", serde_json::to_string(&self.return_dec).unwrap())
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("rpc error")]
    StateRpcError(#[from] RpcError),
    #[error("message code error `{0}`")]
    MsgCodeError(ExitCode),
    #[error("parse return_dec error")]
    ParseReturnDecError(#[from] serde_json::Error),
    #[error("convert return_dec to target error `{0}`")]
    ConvertReturnDecError(String),
}

pub async fn wait_msg<T: FromStr + Default>(rpc: RpcEndpoint, cid: CidJson) -> Result<T, StateError>
    where <T as FromStr>::Err: Debug
{
    let msg_lookup = rpc.post::<_, MessageLookup>(state_api::STATE_WAIT_MSG, json!([cid, 10])).await?;

    if msg_lookup.receipt.exit_code != ExitCode::OK {
        return Err(StateError::MsgCodeError(msg_lookup.receipt.exit_code));
    }

    let ret = serde_json::to_string(&msg_lookup.return_dec)?;
    match T::from_str(&ret) {
        Ok(ret) => Ok(ret),
        Err(err) => Err(StateError::ConvertReturnDecError(format!("{:?}", err))),
    }
}
