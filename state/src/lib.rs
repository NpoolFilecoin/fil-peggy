use forest_blocks::tipset_keys_json::TipsetKeysJson;
use forest_ipld::{json::IpldJson, Ipld};
use forest_json::cid::CidJson;
use forest_rpc_api::state_api;
use fvm_shared::{address::Address, error::ExitCode};
use log::warn;
use rpc::{RpcEndpoint, RpcError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
use thiserror::Error;

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
    pub return_dec: IpldJson,
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
    #[error("rpc error: {0}")]
    StateRpcError(#[from] RpcError),
    #[error("message code error: {0}")]
    MsgCodeError(ExitCode),
    #[error("parse return_dec error: {0}")]
    ParseReturnDecError(#[from] serde_json::Error),
    #[error("convert return_dec to target error: {0}")]
    ConvertReturnDecError(String),
    #[error("parse by yourself {0}")]
    ParseByYourSelf(String),
    #[error("parse address error {0}")]
    ParseAddressError(#[from] fvm_shared::address::Error),
}

pub async fn wait_msg<T: FromStr + Default>(rpc: RpcEndpoint, cid: CidJson) -> Result<T, StateError>
where
    <T as FromStr>::Err: Debug,
{
    let msg_lookup = rpc.post::<_, MessageLookup>(state_api::STATE_WAIT_MSG, json!([cid, 1])).await?;

    if msg_lookup.receipt.exit_code != ExitCode::OK {
        return Err(StateError::MsgCodeError(msg_lookup.receipt.exit_code));
    }

    match msg_lookup.return_dec {
        IpldJson(Ipld::Null) => match msg_lookup.receipt.return_data {
            Some(s) => {
                warn!("> Cannot parse {}, pass to upper", &s);
                Err(StateError::ParseByYourSelf(s))
            }
            None => Ok(T::default()),
        },
        IpldJson(ipld) => {
            let ret = serde_json::to_string(&ipld)?;
            match T::from_str(&ret) {
                Ok(ret) => Ok(ret),
                Err(err) => Err(StateError::ConvertReturnDecError(format!("{:?}", err))),
            }
        }
    }
}

pub async fn lookup_id(rpc: RpcEndpoint, addr: Address) -> Result<Address, StateError> {
    // state_api::STATE_LOOKUP_ID definition is wrong
    match rpc
        .post::<_, String>(
            /* state_api::STATE_LOOKUP_ID */ "Filecoin.StateLookupID",
            json!([addr.to_string(), []]),
        )
        .await
    {
        Ok(addr) => Ok(Address::from_str(&addr)?),
        Err(err) => Err(StateError::StateRpcError(err)),
    }
}
