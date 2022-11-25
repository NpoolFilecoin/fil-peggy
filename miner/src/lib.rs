use forest_key_management::{
    KeyInfo,
};
use fvm_shared::{
    address::Address,
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
    BytesDe,
};
use fil_actors_runtime::STORAGE_POWER_ACTOR_ADDR;
use forest_json::{
    cid::CidJson,
};
use rpc::RpcEndpoint;
use serde::Deserialize;
use thiserror::Error;
use std::str::FromStr;
use mpool::{mpool_push, MpoolError};

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
    #[error("mpool call error")]
    MpoolCallError(#[from] MpoolError)
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
    pub async fn create_miner(&self) -> Result<CidJson, MinerError> {
        let params = CreateMinerParams {
            owner: self.owner,
            worker: self.worker,
            window_post_proof_type: self.window_post_proof_type,
            peer: self.peer_id.to_bytes(),
            multiaddrs: vec![BytesDe("peggy-miner".as_bytes().to_vec())],
        };
        match mpool_push::<_, CidJson>(
            self.rpc.clone(),
            self.owner,
            self.owner_key_info.clone(),
            STORAGE_POWER_ACTOR_ADDR,
            2,
            TokenAmount::from_atto(0),
            params,
        ).await {
            Ok(res) => Ok(res),
            Err(err) => Err(MinerError::MpoolCallError(err)),
        }
    }

    pub fn change_owner(&self) -> Result<String, &str> {
        Ok(String::default())
    }
}
