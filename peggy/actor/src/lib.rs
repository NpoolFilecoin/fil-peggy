use anyhow::{anyhow, Error as AnyhowError};
use base64;
use cid::Cid;
use fil_actor_init::{ExecParams, ExecReturn as ExecReturn1, InstallParams, InstallReturn as InstallReturn1};
use fil_actors_runtime::INIT_ACTOR_ADDR;
use forest_json::cid::CidJson;
use forest_key_management::KeyInfo;
use fvm_ipld_encoding_3::{
    tuple::{Deserialize_tuple, Serialize_tuple},
    Cbor,
    RawBytes,
};
use fvm_shared::{address::Address, econ::TokenAmount};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
    string::FromUtf8Error,
};
use thiserror::Error;

use mpool::{mpool_push, MpoolError};
use rpc::RpcEndpoint;
use state::{wait_msg, StateError};

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("io call error: {0}")]
    IOCallError(#[from] std::io::Error),
    #[error("common error: {0}")]
    CommonError(#[from] AnyhowError),
    #[error("parse json error: {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("parse utf8 error: {0}")]
    ParseUtf8Error(#[from] FromUtf8Error),
    #[error("mpool call error: {0}")]
    MpoolCallError(#[from] MpoolError),
    #[error("state call error: {0}")]
    StateCallError(#[from] StateError),
    #[error("decode base64 error: {0}")]
    DecodeBase64Error(#[from] base64::DecodeError),
    #[error("decode ipld error: {0}")]
    DecodeIpldError(#[from] fvm_ipld_encoding_3::Error),
    #[error("parse address error")]
    ParseAddressError(#[from] fvm_shared::address::Error),
    #[error("clone fvm repo error code {0}")]
    CloneFVMRepoError(std::process::ExitStatus),
}

pub fn clone_actor(repo_url: &str, repo_rev: &str, target_path: PathBuf) -> Result<(), ActorError> {
    let out = Command::new("git")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("clone")
        .arg("--single-branch")
        .arg("--branch")
        .arg(repo_rev)
        .arg(repo_url)
        .arg(target_path.display().to_string())
        .output()?;
    if !out.status.success() {
        return Err(ActorError::CloneFVMRepoError(out.status));
    }

    Ok(())
}

pub fn compile_actor(target_path: PathBuf) -> Result<PathBuf, ActorError> {
    Command::new("cargo")
        .current_dir(target_path.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("build")
        .arg("--release")
        .output()?;

    let output = Command::new("cargo").current_dir(target_path.clone()).arg("read-manifest").output()?;

    if !output.status.success() {
        return Err(ActorError::CommonError(anyhow!("fail read manifest")));
    }

    let manifest = String::from_utf8(output.stdout)?;
    let value = serde_json::from_str::<serde_json::Value>(&manifest)?;
    let name = value.get("name").ok_or(ActorError::CommonError(anyhow!("invalid name")))?;
    let name = name.as_str().ok_or(ActorError::CommonError(anyhow!("invalid name")))?;

    let wasm_path = target_path.join("target/release/wbuild");
    let wasm_path = wasm_path.join(name);
    let mut wasm_path = wasm_path.join(name);
    wasm_path.set_extension("compact.wasm");

    Ok(wasm_path)
}

#[derive(Serialize, Deserialize)]
pub struct InstallReturn {
    pub code_cid: CidJson,
    pub installed: bool,
}

impl FromStr for InstallReturn {
    type Err = ActorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<Self>(s) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ActorError::ParseJsonError(err)),
        }
    }
}

impl Default for InstallReturn {
    fn default() -> Self {
        Self {
            code_cid: CidJson(
                Cid::from_str("bafyreibjo4xmgaevkgud7mbifn3dzp4v4lyaui4yvqp3f2bqwtxcjrdqg4").unwrap() as Cid
            ),
            installed: false,
        }
    }
}

pub async fn install_actor(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    target_path: PathBuf,
) -> Result<(CidJson, bool), ActorError> {
    let code = std::fs::read(target_path)?;
    let code = RawBytes::from(code);
    let params = InstallParams { code };

    match mpool_push::<_, CidJson>(
        rpc.clone(),
        from,
        from_key_info,
        INIT_ACTOR_ADDR,
        4,
        TokenAmount::from_atto(0),
        params,
    )
    .await
    {
        Ok(res) => match wait_msg::<InstallReturn>(rpc, res.clone()).await {
            Ok(ret) => Ok((ret.code_cid, ret.installed)),
            Err(StateError::ParseByYourSelf(s)) => {
                warn!("> State cannot parse {}, parse by youself!", &s);
                let s = base64::decode_config(&s, base64::STANDARD)?;
                let b = RawBytes::new(s);
                let ret: InstallReturn1 = RawBytes::deserialize(&b)?;
                Ok((CidJson(ret.code_cid), ret.installed))
            }
            Err(err) => Err(ActorError::StateCallError(err)),
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExecReturn {
    #[serde(rename = "IDAddress")]
    pub id_address: Address,
    pub robust_address: Address,
}

impl FromStr for ExecReturn {
    type Err = ActorError;

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

pub async fn create_actor(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    actor_code_id: CidJson,
) -> Result<(Address, Address), ActorError> {
    let CidJson(_cid) = actor_code_id;
    let params = ExecParams { code_cid: _cid, constructor_params: RawBytes::new(Vec::new()) };

    match mpool_push::<_, CidJson>(
        rpc.clone(),
        from,
        from_key_info,
        INIT_ACTOR_ADDR,
        2,
        TokenAmount::from_atto(0),
        params,
    )
    .await
    {
        Ok(res) => match wait_msg::<ExecReturn>(rpc, res.clone()).await {
            Ok(ret) => Ok((
                Address::from_str(&ret.id_address.to_string())?,
                Address::from_str(&ret.robust_address.to_string())?,
            )),
            Err(StateError::ParseByYourSelf(s)) => {
                warn!("> State cannot parse {}, parse by youself!", &s);
                let s = base64::decode_config(&s, base64::STANDARD)?;
                let b = RawBytes::new(s);
                let ret: ExecReturn1 = RawBytes::deserialize(&b)?;
                Ok((
                    Address::from_str(&ret.id_address.to_string())?,
                    Address::from_str(&ret.robust_address.to_string())?,
                ))
            }
            Err(err) => Err(ActorError::StateCallError(err)),
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

pub async fn take_owner(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    actor_id: Address,
    miner_id: Address,
) -> Result<(), ActorError> {
    match mpool_push::<_, CidJson>(rpc.clone(), from, from_key_info, actor_id, 16, TokenAmount::from_atto(0), miner_id)
        .await
    {
        Ok(res) => match wait_msg::<serde_json::Value>(rpc, res).await {
            Ok(_) => Ok(()),
            Err(StateError::ParseByYourSelf(s)) => {
                warn!("> State cannot parse {}, parse by youself!", &s);
                let s = base64::decode_config(&s, base64::STANDARD)?;
                info!("> {}", String::from_utf8(s)?);
                Ok(())
            }
            Err(err) => Err(ActorError::StateCallError(err)),
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Default)]
struct ChangeWorkerParams {
    miner_id: Address,
    new_worker_id: Address,
}
impl Cbor for ChangeWorkerParams {}

pub async fn change_worker(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    actor_id: Address,
    miner_id: Address,
    new_worker_id: Address,
) -> Result<(), ActorError> {
    let params = ChangeWorkerParams { miner_id, new_worker_id };

    match mpool_push::<_, CidJson>(rpc.clone(), from, from_key_info, actor_id, 18, TokenAmount::from_atto(0), params)
        .await
    {
        Ok(res) => match wait_msg::<serde_json::Value>(rpc, res).await {
            Ok(_) => Ok(()),
            Err(StateError::ParseByYourSelf(s)) => {
                warn!("> State cannot parse {}, parse by youself!", &s);
                let s = base64::decode_config(&s, base64::STANDARD)?;
                info!("> {}", String::from_utf8(s)?);
                Ok(())
            }
            Err(err) => Err(ActorError::StateCallError(err)),
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Default)]
struct WithdrawMinerParams {
    miner_id: Address,
    amount: TokenAmount,
}
impl Cbor for WithdrawMinerParams {}

pub async fn withdraw_miner(
    rpc: RpcEndpoint,
    from: Address,
    from_key_info: KeyInfo,
    actor_id: Address,
    miner_id: Address,
    amount: TokenAmount,
) -> Result<(), ActorError> {
    let params = WithdrawMinerParams { miner_id, amount };

    match mpool_push::<_, CidJson>(rpc.clone(), from, from_key_info, actor_id, 19, TokenAmount::from_atto(0), params)
        .await
    {
        Ok(res) => match wait_msg::<serde_json::Value>(rpc, res).await {
            Ok(_) => Ok(()),
            Err(StateError::ParseByYourSelf(s)) => {
                warn!("> State cannot parse {}, parse by youself!", &s);
                let s = base64::decode_config(&s, base64::STANDARD)?;
                info!("> {}", String::from_utf8(s)?);
                Ok(())
            }
            Err(err) => Err(ActorError::StateCallError(err)),
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}
