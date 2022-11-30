use forest_json::signed_message::json::SignedMessageJson;
use forest_key_management::KeyInfo;
use forest_message::signed_message::SignedMessage;
use forest_rpc_api::mpool_api;
use fvm_ipld_encoding::{Cbor, RawBytes};
use fvm_shared::{address::Address, econ::TokenAmount, message::Message};
use gasestimator::{estimate_msg_gas, GasEstimatorError};
use log::error;
use num_bigint::BigInt;
use rpc::{RpcEndpoint, RpcError};
use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};
use thiserror::Error;
use wallet::{get_balance, WalletError};

#[derive(Error, Debug)]
pub enum MpoolError {
    #[error("rpc request error: {0}")]
    RpcRequestError(#[from] RpcError),
    #[error("fvm ipld encoding error: {0}")]
    FvmIpldEncodingError(#[from] fvm_ipld_encoding::Error),
    #[error("key management error: {0}")]
    KeyManagementError(#[from] forest_key_management::Error),
    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("gas estimator error: {0}")]
    EstimateGasError(#[from] GasEstimatorError),
    #[error("wallet call error: {0}")]
    WalletCallError(#[from] WalletError),
    #[error("insufficient funds")]
    InsufficientFunds,
}

async fn mpool_get_nonce(rpc: RpcEndpoint, address: Address) -> Result<u64, MpoolError> {
    match rpc.post::<_, u64>(mpool_api::MPOOL_GET_NONCE, vec![address.to_string()]).await {
        Ok(res) => Ok(res),
        Err(err) => Err(MpoolError::RpcRequestError(err)),
    }
}

pub async fn mpool_push<T1: serde::Serialize, T2: for<'de> serde::Deserialize<'de>>(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    to: Address,
    method_num: u64,
    value: TokenAmount,
    params: T1,
) -> Result<T2, MpoolError> {
    let nonce = mpool_get_nonce(rpc.clone(), from).await?;
    let balance = get_balance(rpc.clone(), from).await?;

    let params = RawBytes::serialize(params)?;
    let msg = Message {
        version: 0,
        to,
        from,
        method_num,
        value: value.clone(),
        sequence: nonce,
        params,
        gas_fee_cap: TokenAmount::from_atto(0),
        gas_limit: 0,
        gas_premium: TokenAmount::from_atto(0),
    };

    let msg = estimate_msg_gas(rpc.clone(), msg.clone()).await?;

    let gas_fee = msg.clone().gas_fee_cap.add(msg.clone().gas_premium.mul(BigInt::from(msg.clone().gas_limit)));
    if balance.cmp(&gas_fee.clone().add(value.clone())) == Ordering::Less {
        error!("Account {} balance {} < fee {} + value {}", from, balance, gas_fee, value);
        return Err(MpoolError::InsufficientFunds);
    }

    let msg_cid = msg.cid()?;
    let sig = forest_key_management::sign(
        *from_key_info.key_type(),
        from_key_info.private_key(),
        msg_cid.to_bytes().as_slice(),
    )?;
    let smsg = SignedMessage::new_from_parts(msg, sig)?;

    match rpc.post::<_, T2>(mpool_api::MPOOL_PUSH, vec![SignedMessageJson(smsg.clone())]).await {
        Ok(res) => Ok(res),
        Err(err) => Err(MpoolError::RpcRequestError(err)),
    }
}
