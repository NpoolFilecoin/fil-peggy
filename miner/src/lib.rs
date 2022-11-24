use forest_key_management::{
    KeyInfo,
};
use fvm_shared::{
    address::Address,
    message::Message,
    econ::TokenAmount,
    sector::RegisteredPoStProof,
};
use fil_actor_power::{
    CreateMinerParams,
};
use libp2p::{
    PeerId,
};
use fvm_ipld_encoding::{
    RawBytes,
    BytesDe,
    Cbor,
};
use fil_actors_runtime::STORAGE_POWER_ACTOR_ADDR;
use forest_message::signed_message::SignedMessage;
use forest_json::{
    signed_message::json::SignedMessageJson,
    cid::CidJson,
};
use forest_rpc_api::{
    mpool_api,
};
use rpc::{RpcEndpoint, RpcError};
use serde::Deserialize;
use thiserror::Error;
use std::str::FromStr;

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("miss IDAddress in string")]
    MissIDAddress,
    #[error("miss RobustAddress in string")]
    MissRobustAddress,
    #[error("json parse error")]
    JsonParseError(#[from] serde_json::Error),
    #[error("address parse error")]
    AddressParseError(#[from] fvm_shared::address::Error),
}

pub struct CreateMinerReturn {
    pub id_address: Address,
    pub robust_address: Address,
}

impl FromStr for CreateMinerReturn {
    type Err = MinerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct T {
            #[serde(rename = "IDAddress")]
            id_address: String,
            robust_address: String,
        }

        let v = serde_json::from_str::<T>(s)?;

        let id_address = Address::from_str(&v.id_address)?;
        let robust_address = Address::from_str(&v.robust_address)?;

        Ok(Self {
            id_address,
            robust_address,
        })
    }
}

pub struct Miner {
    pub owner: Address,
    pub owner_key_info: KeyInfo,
    pub worker: Address,
    pub window_post_proof_type: RegisteredPoStProof,
    pub peer_id: PeerId,
    pub rpc: RpcEndpoint,
    pub miner_id: Option<Address>,
    pub multiaddrs: Option<Vec<BytesDe>>,
}

impl Miner {
    pub async fn create_miner(&self) -> Result<CidJson, RpcError> {
        let key_info = self.owner_key_info.clone();

        let params = CreateMinerParams {
            owner: self.owner,
            worker: self.worker,
            window_post_proof_type: self.window_post_proof_type,
            peer: self.peer_id.to_bytes(),
            multiaddrs: vec![BytesDe("peggy-miner".as_bytes().to_vec())],
        };
        let params = RawBytes::serialize(params).unwrap();
        let msg = Message {
            version: 0,
            to: STORAGE_POWER_ACTOR_ADDR,
            from: self.owner,
            method_num: 2,
            value: TokenAmount::from_atto(0),
            sequence: 0,
            params: params,
            gas_fee_cap: TokenAmount::from_atto(101137),
            gas_limit: 32932877,
            gas_premium: TokenAmount::from_atto(100083),
        };
        let msg_cid = msg.cid().unwrap();
        let sig = forest_key_management::sign(
            *key_info.key_type(),
            key_info.private_key(),
            msg_cid.to_bytes().as_slice(),
            ).unwrap();
        let smsg = SignedMessage::new_from_parts(msg, sig).unwrap();

        self.rpc
            .post::<_, CidJson>(mpool_api::MPOOL_PUSH, vec![SignedMessageJson(smsg.clone())])
            .await
    }

    pub fn change_owner(&self) -> Result<String, &str> {
        Ok(String::default())
    }
}
