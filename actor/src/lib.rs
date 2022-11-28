use git2::{self, Repository};
use thiserror::Error;
use std::path::PathBuf;
use anyhow::{anyhow, Error as AnyhowError};
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;
use fil_actor_init::{
    InstallParams,
    InstallReturn as InstallReturn1,
    ExecParams,
    ExecReturn as ExecReturn1,
};
use fil_actors_runtime::INIT_ACTOR_ADDR;
use forest_key_management::KeyInfo;
use fvm_shared::{
    econ::TokenAmount,
    address::Address,
};
use fvm_ipld_encoding_3::{
    RawBytes,
};
use forest_json::{
    cid::CidJson,
};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use cid::Cid;
use base64;
use log::warn;

use mpool::{mpool_push, MpoolError};
use rpc::RpcEndpoint;
use state::{wait_msg, StateError};

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("git call error: {0}")]
    GitCallError(#[from] git2::Error),
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
}

pub fn clone_actor(repo_url: &str, target_path: PathBuf) -> Result<(), ActorError> {
    let _ = Repository::clone(repo_url, target_path)?;
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

    let output = Command::new("cargo")
        .current_dir(target_path.clone())
        .arg("read-manifest")
        .output()?;

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
            code_cid: CidJson(Cid::from_str("bafyreibjo4xmgaevkgud7mbifn3dzp4v4lyaui4yvqp3f2bqwtxcjrdqg4").unwrap() as Cid),
            installed: false,
        }
    }
}

pub async fn install_actor(
    rpc: RpcEndpoint,
    target_path: PathBuf,
    from: Address,
    from_key_info: KeyInfo,
) -> Result<(CidJson, bool), ActorError> {
    let code = std::fs::read(target_path)?;
    let code = RawBytes::from(code);
    let params = InstallParams {
        code: code,
    };

    match mpool_push::<_, CidJson>(
        rpc.clone(),
        from,
        from_key_info,
        INIT_ACTOR_ADDR,
        4,
        TokenAmount::from_atto(0),
        params,
    ).await {
        Ok(res) => {
            match wait_msg::<InstallReturn>(rpc, res.clone()).await {
                Ok(ret) => Ok((ret.code_cid, ret.installed)),
                Err(StateError::ParseByYourSelf(s)) => {
                    warn!("> State cannot parse {}, parse by youself!", &s);
                    let s = base64::decode_config(&s, base64::STANDARD)?;
                    let b = RawBytes::new(s);
                    let ret: InstallReturn1 = RawBytes::deserialize(&b)?;
                    Ok((CidJson(ret.code_cid), ret.installed))
                },
                Err(err) => Err(ActorError::StateCallError(err)),
            }
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExecReturn {
    pub id_address: Address,
    pub robust_address: Address,
}

impl FromStr for ExecReturn {
    type Err = ActorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<Self>(s) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(ActorError::ParseJsonError(err)),
        }
    }
}

pub async fn create_actor(
    rpc: RpcEndpoint,
    actor_code_id: CidJson,
    from: Address,
    from_key_info: KeyInfo,
) -> Result<(Address, Address), ActorError> {
    let CidJson(_cid) = actor_code_id;
    let params = ExecParams {
        code_cid: _cid,
        constructor_params: RawBytes::new(Vec::new()),
    };

    match mpool_push::<_, CidJson>(
        rpc.clone(),
        from,
        from_key_info,
        INIT_ACTOR_ADDR,
        2,
        TokenAmount::from_atto(0),
        params,
    ).await {
        Ok(res) => {
            match wait_msg::<ExecReturn>(rpc, res.clone()).await {
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
                },
                Err(err) => Err(ActorError::StateCallError(err)),
            }
        },
        Err(err) => Err(ActorError::MpoolCallError(err)),
    }
}

pub fn take_owner() {
    println!("{}", " Try take owner");
}
