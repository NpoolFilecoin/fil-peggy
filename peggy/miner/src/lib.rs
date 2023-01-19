use fil_actor_power::CreateMinerParams;
use fil_actors_runtime::STORAGE_POWER_ACTOR_ADDR;
use forest_json::cid::CidJson;
use forest_key_management::KeyInfo;
use fvm_ipld_encoding::BytesDe;
use fvm_shared::{address::Address, econ::TokenAmount, sector::RegisteredPoStProof};
use libp2p::PeerId;
use multiaddr::Multiaddr;
use rpc::RpcEndpoint;
use serde::Deserialize;
use serde_json;
use std::str::FromStr;
use thiserror::Error;

use mpool::{mpool_push, MpoolError};
use state::{wait_msg, StateError};

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("miss IDAddress in string")]
    MissIDAddress,
    #[error("miss RobustAddress in string")]
    MissRobustAddress,
    #[error("json parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("address parse error: {0}")]
    AddressParseError(#[from] fvm_shared::address::Error),
    #[error("mpool call error: {0}")]
    MpoolCallError(#[from] MpoolError),
    #[error("state call error: {0}")]
    StateCallError(#[from] StateError),
    #[error("parse multiaddr error: {0}")]
    ParseMultiaddrError(#[from] multiaddr::Error),
}

#[derive(Default)]
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

        Ok(Self { id_address, robust_address })
    }
}

pub async fn create_miner(
    rpc: RpcEndpoint,
    owner: Address,
    owner_key_info: KeyInfo,
    worker: Address,
    window_post_proof_type: RegisteredPoStProof,
    peer_id: PeerId,
) -> Result<(Address, Address), MinerError> {
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/2345/http".parse()?;
    let params = CreateMinerParams {
        owner,
        worker,
        window_post_proof_type,
        peer: peer_id.to_bytes(),
        multiaddrs: vec![BytesDe(addr.to_vec())],
    };

    match mpool_push::<_, CidJson>(
        rpc.clone(),
        owner,
        owner_key_info,
        STORAGE_POWER_ACTOR_ADDR,
        2,
        TokenAmount::from_atto(0),
        params,
    )
    .await
    {
        Ok(res) => match wait_msg::<CreateMinerReturn>(rpc.clone(), res.clone()).await {
            Ok(ret) => Ok((ret.id_address, ret.robust_address)),
            Err(err) => Err(MinerError::StateCallError(err)),
        },
        Err(err) => Err(MinerError::MpoolCallError(err)),
    }
}

pub async fn change_owner(
    rpc: RpcEndpoint,
    owner: Address,
    owner_key_info: KeyInfo,
    miner_id: Address,
    new_owner_id: Address,
) -> Result<(), MinerError> {
    match mpool_push::<_, CidJson>(
        rpc.clone(),
        owner,
        owner_key_info,
        miner_id,
        23,
        TokenAmount::from_atto(0),
        new_owner_id,
    )
    .await
    {
        Ok(res) => match wait_msg::<serde_json::Value>(rpc, res).await {
            Ok(_) => Ok(()),
            Err(err) => Err(MinerError::StateCallError(err)),
        },
        Err(err) => Err(MinerError::MpoolCallError(err)),
    }
}
