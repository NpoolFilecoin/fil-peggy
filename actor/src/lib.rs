use clap::Parser;
use scanf::scanf;
use std::str::FromStr;
use colored::Colorize;
use forest_key_management::{
    json::KeyInfoJson,
    KeyInfo,
};
use fvm_shared::{
    crypto::signature::SignatureType,
    address::Address,
    sector::{RegisteredSealProof, SectorSize},
    version::NetworkVersion,
    message::Message,
    econ::TokenAmount,
};
use fil_actor_power::{
    CreateMinerParams,
};
use libp2p::{
    identity::{ed25519, Keypair},
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
use reqwest::{
    blocking,
    header::{CONTENT_TYPE, AUTHORIZATION},
};
use jsonrpc_v2::RequestObject;
use forest_rpc_api::{
    mpool_api,
    state_api,
};
use serde_json::json;

fn compile_actor() {
    println!("{}", " Try compile actor".yellow());
}

fn deploy_actor() {
    println!("{}", " Try deploy actor".yellow());
}

fn create_actor() {
    println!("{}", " Try create actor".yellow());
}

fn take_owner() {
    println!("{}", " Try take owner".yellow());
}
