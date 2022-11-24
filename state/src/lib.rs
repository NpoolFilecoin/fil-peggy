use rpc::RpcEndpoint;
use forest_json::{
    cid::CidJson,
    // message_receipt::json::ReceiptJson,
};
use forest_rpc_api::{
    state_api,
};
use serde::{Deserialize, Serialize};
// use forest_ipld::json::IpldJson;
use forest_blocks::{
    tipset_keys_json::TipsetKeysJson,
};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MessageLookup {
    // pub receipt: ReceiptJson,
    #[serde(rename = "TipSet")]
    pub tipset: TipsetKeysJson,
    pub height: i64,
    pub message: CidJson,
    // pub return_dec: IpldJson,
}

pub async fn wait_msg(rpc: RpcEndpoint, cid: CidJson) -> Result<MessageLookup, String> {
    rpc.post::<_, MessageLookup>(state_api::STATE_WAIT_MSG, json!([cid, 10])).await
}
