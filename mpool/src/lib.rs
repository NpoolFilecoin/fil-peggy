use rpc::{RpcEndpoint, RpcError};
use thiserror::Error;
use forest_key_management::{
    KeyInfo,
};
use fvm_shared::{
    address::Address,
    message::Message,
    econ::TokenAmount,
};
use fvm_ipld_encoding::{
    RawBytes,
    Cbor,
};
use forest_message::signed_message::SignedMessage;
use forest_json::{
    signed_message::json::SignedMessageJson,
};
use forest_rpc_api::{
    mpool_api,
};

#[derive(Error, Debug)]
pub enum MpoolError {
    #[error("rpc request error")]
    RpcRequestError(#[from] RpcError),
    #[error("fvm ipld encoding error")]
    FvmIpldEncodingError(#[from] fvm_ipld_encoding::Error),
    #[error("key management error")]
    KeyManagementError(#[from] forest_key_management::Error),
    #[error("anyhow error")]
    AnyhowError(#[from] anyhow::Error)
}

pub async fn mpool_push<
    T1: serde::Serialize,
    T2: for<'de>serde::Deserialize<'de>>(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    to: Address,
    method_num: u64,
    value: TokenAmount,
    params: T1) -> Result<T2, MpoolError>
{
    // TODO: get nonce
    // TODO: estimate gas
    // TODO: check balance

    let params = RawBytes::serialize(params)?;
    let msg = Message {
        version: 0,
        to: to,
        from: from,
        method_num: method_num,
        value: value,
        sequence: 0,
        params: params,
        gas_fee_cap: TokenAmount::from_atto(101137),
        gas_limit: 32932877,
        gas_premium: TokenAmount::from_atto(100083),
    };
    let msg_cid = msg.cid()?;
    let sig = forest_key_management::sign(
        *from_key_info.key_type(),
        from_key_info.private_key(),
        msg_cid.to_bytes().as_slice())?;
    let smsg = SignedMessage::new_from_parts(msg, sig)?;

    match rpc.post::<_, T2>(
        mpool_api::MPOOL_PUSH,
        vec![SignedMessageJson(smsg.clone())],
    ).await {
        Ok(res) => Ok(res),
        Err(err) => Err(MpoolError::RpcRequestError(err)),
    }
}
