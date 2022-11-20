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
use fil_actor_power::CreateMinerParams;
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
};
use reqwest::blocking;
use jsonrpc_v2::RequestObject;
use forest_rpc_api::mpool_api;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
}

#[derive(Debug)]
enum MenuItem {
    Wallet,
    Miner,
    Actor,
}

impl FromStr for MenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            1 => Ok(Self::Wallet),
            2 => Ok(Self::Miner),
            3 => Ok(Self::Actor),
            _ => Ok(Self::Wallet),
        }
    }
}

fn select_menu() -> Result<MenuItem, String> {
    println!("{}", "Action you want:".green());
    println!("{}{}", "  1".green(), ". Wallet".blue());
    println!("{}{}", "  2".green(), ". Miner".blue());
    println!("{}{}", "  3".green(), ". Actor".blue());

    let mut action = MenuItem::Wallet;
    match scanf!("{}", action) {
        Ok(_) => {
            Ok(action)
        },
        Err(err) => Err(err.to_string()),
    }
}

enum WalletMenuItem {
    Secp256k1,
    BLS,
}

impl FromStr for WalletMenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            1 => Ok(Self::Secp256k1),
            2 => Ok(Self::BLS),
            _ => Ok(Self::Secp256k1),
        }
    }
}

fn create_wallet(wallet_type: SignatureType) {
    let key = forest_key_management::generate_key(wallet_type).unwrap();
    println!("{}", " Create new wallet: ".yellow());
    println!("{}{}", "   Address: ".yellow(), key.address.to_string());

    let key_info = KeyInfoJson(key.key_info);
    let encoded_key = serde_json::to_string(&key_info).unwrap();
    let encoded_key = hex::encode(encoded_key);

    println!("{}{:?}", "   KeyInfo: ".yellow(), encoded_key);
}

fn wallet_handler() {
    println!("{}", "Wallet action you want:".green());
    println!("{}{}", "  1".green(), ". Secp256k1".blue());
    println!("{}{}", "  2".green(), ". BLS".blue());

    let mut wallet_type = WalletMenuItem::Secp256k1;
    match scanf!("{}", wallet_type) {
        Ok(_) => {
            let wallet_type = match wallet_type {
                WalletMenuItem::Secp256k1 => SignatureType::Secp256k1,
                WalletMenuItem::BLS => SignatureType::BLS,
            };
            create_wallet(wallet_type)
        },
        Err(err) => {
            println!("{}", format!("  Fail to get wallet type input: {}", err).red());
        },
    }
}

enum MinerMenuItem {
    Create,
    ChangeOwner,
}

impl FromStr for MinerMenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            1 => Ok(Self::Create),
            2 => Ok(Self::ChangeOwner),
            _ => Ok(Self::Create),
        }
    }
}

fn create_miner() {
    let mut owner: Address = Address::default();
    println!("{}", "Enter miner's owner address:".green());
    scanf!("{}", owner).unwrap();
    println!("{}{}{}", "  You will use ".yellow(), owner, " as owner address.".yellow());

    println!("{}", "Enter owner's key info:".green());
    let mut key_info = String::default();
    scanf!("{}", key_info).unwrap();
    let key_info = hex::decode(&key_info).unwrap();
    let key_info: KeyInfoJson = serde_json::from_slice(&key_info).unwrap();
    println!("{}{:?}", "  KeyInfoJson: ".yellow(), key_info);
    let key_info: KeyInfo = KeyInfo::from(key_info);

    let mut worker: Address = Address::default();
    println!("{}", "Enter miner's worker address:".green());
    scanf!("{}", worker).unwrap();
    println!("{}{}{}", "  You will use ".yellow(), worker, " as worker address.".yellow());

    println!("{}", "Enter miner's sector size:".green());
    println!("{}{}", "  1".green(), ". 32GiB".blue());
    println!("{}{}", "  2".green(), ". 64GiB".blue());
    println!("{}{}", "  3".green(), ". 2KiB".blue());

    let mut proof_type = 0;
    scanf!("{}", proof_type).unwrap();
    let sector_size =  match proof_type {
        1 => SectorSize::_32GiB,
        2 => SectorSize::_64GiB,
        3 => SectorSize::_2KiB,
        _ => SectorSize::_32GiB,
    };

    let seal_proof = RegisteredSealProof::from_sector_size(sector_size, NetworkVersion::V17);
    let post_proof = seal_proof.registered_window_post_proof().unwrap();
    println!("{}{:?}{}", "  You will use ".yellow(), post_proof, " as miner post proof.".yellow());

    let gen_keypair = ed25519::Keypair::generate();
    let net_keypair = Keypair::Ed25519(gen_keypair);
    println!("{}", "  You peer id:".green());
    println!("{}{:?}", "    Public key: ".yellow(), net_keypair.public());
    println!("{}{:?}", "    Key pair: ".yellow(), net_keypair);
    let peer_id = PeerId::from(net_keypair.public());
    println!("{}{:?}", "    PeerId:".yellow(), peer_id);

    let mut rpc_host = String::from("https://wallaby.node.glif.io/rpc/v1");
    println!("{}{}", "Enter wallet rpc endpoint:".green(), format!(" (default https://wallaby.node.glif.io/rpc/v1)"));
    scanf!("{}", rpc_host).unwrap();
    println!("{}{}{}", "  You will use ".yellow(), rpc_host, " as your rpc host.".yellow());

    let params = CreateMinerParams {
        owner,
        worker,
        window_post_proof_type: post_proof,
        peer: peer_id.to_bytes(),
        multiaddrs: vec![BytesDe("".as_bytes().to_vec())],
    };
    let params = RawBytes::serialize(params).unwrap();
    let msg = Message {
        version: 0,
        to: STORAGE_POWER_ACTOR_ADDR,
        from: owner,
        method_num: 2,
        value: TokenAmount::from_atto(1000),
        sequence: 0,
        params: params,
        gas_fee_cap: TokenAmount::from_nano(100000),
        gas_limit: 0,
        gas_premium: TokenAmount::from_atto(1000),
    };
    let msg_cid = msg.cid().unwrap();
    let sig = forest_key_management::sign(
        *key_info.key_type(),
        key_info.private_key(),
        msg_cid.to_bytes().as_slice(),
    ).unwrap();
    let smsg = SignedMessage::new_from_parts(msg, sig).unwrap();
    let signed_msg = serde_json::to_string(&SignedMessageJson(smsg.clone())).unwrap();
    println!("{}", "  Create miner message:".green());
    println!("{}{:?}", "    CID:".yellow(), msg_cid);
    println!("{}{:?}", "    MSG:".yellow(), signed_msg);

    let req = RequestObject::request()
        .with_params(serde_json::to_value(smsg.clone()).unwrap())
        .with_method(mpool_api::MPOOL_PUSH)
        .with_id(7878)
        .finish();
    let req = serde_json::to_string(&req).unwrap();

    let cli = blocking::Client::new();
    let res = cli
        .post(rpc_host)
        .body(req.clone())
        .send()
        .unwrap();
    println!("{}{:?}", "Create miner: ".yellow(), res);
    println!("{}{}", "  Input: ".yellow(), req);
}

fn change_owner() {

}

fn miner_handler() {
    println!("{}", "Miner action you want:".green());
    println!("{}{}", "  1".green(), ". Create".blue());
    println!("{}{}", "  2".green(), ". ChangeOwner".blue());

    let mut action = MinerMenuItem::Create;
    match scanf!("{}", action) {
        Ok(_) => {
            match action {
                MinerMenuItem::Create => create_miner(),
                MinerMenuItem::ChangeOwner => change_owner(),
            }
        },
        Err(err) => {
            println!("{}", format!("  Fail to get miner menu input: {}", err).red());
        },
    }
}

enum ActorMenuItem {
    Compile,
    Deploy,
    Create,
    TakeOwner,
}

impl FromStr for ActorMenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            1 => Ok(Self::Compile),
            2 => Ok(Self::Deploy),
            3 => Ok(Self::Create),
            4 => Ok(Self::TakeOwner),
            _ => Ok(Self::Compile),
        }
    }
}

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

fn actor_handler() {
    println!("{}", "Actor action you want:".green());
    println!("{}{}", "  1".green(), ". Compile".blue());
    println!("{}{}", "  2".green(), ". Deploy".blue());
    println!("{}{}", "  3".green(), ". Create".blue());
    println!("{}{}", "  4".green(), ". TakeOwner".blue());

    let mut action = ActorMenuItem::Compile;
    match scanf!("{}", action) {
        Ok(_) => {
            match action {
                ActorMenuItem::Compile => compile_actor(),
                ActorMenuItem::Deploy => deploy_actor(),
                ActorMenuItem::Create => create_actor(),
                ActorMenuItem::TakeOwner => take_owner(),
            }
        },
        Err(err) => {
            println!("{}", format!("  Fail to get actor menu input: {}", err).red());
        },
    }
}

pub fn cli_main() {
    let _ = Args::parse();

    loop {
        let menu = select_menu().unwrap();
        match menu {
            MenuItem::Wallet => wallet_handler(),
            MenuItem::Miner => miner_handler(),
            MenuItem::Actor => actor_handler(),
        }
    }
}
