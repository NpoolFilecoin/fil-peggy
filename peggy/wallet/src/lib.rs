use forest_key_management::{json::KeyInfoJson, Key};
use forest_rpc_api::wallet_api;
use fvm_shared::{address::Address, crypto::signature::SignatureType, econ::TokenAmount};
use num_bigint::{BigInt, ParseBigIntError};
use rpc::{RpcEndpoint, RpcError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("rpc request error")]
    RpcRequestError(#[from] RpcError),
    #[error("parse token amount error")]
    ParseTokenAmountError(#[from] ParseBigIntError),
}

pub fn create_wallet(wallet_type: SignatureType) -> (Address, String, Key, KeyInfoJson) {
    let key = forest_key_management::generate_key(wallet_type).unwrap();
    let key_info = KeyInfoJson(key.clone().key_info);
    let encoded_key = serde_json::to_string(&key_info).unwrap();
    let encoded_key = hex::encode(encoded_key);
    (key.address, encoded_key, key, key_info)
}

pub async fn get_balance(rpc: RpcEndpoint, address: Address) -> Result<TokenAmount, WalletError> {
    match rpc.post::<_, String>(wallet_api::WALLET_BALANCE, vec![address.to_string()]).await {
        Ok(res) => {
            let res = res.parse::<BigInt>()?;
            let res = TokenAmount::from_atto(res);
            Ok(res)
        }
        Err(err) => Err(WalletError::RpcRequestError(err)),
    }
}
