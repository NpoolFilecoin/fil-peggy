use clap::{Parser, Subcommand};
use core::str::FromStr;
use scanf::scanf;
use colored::Colorize;
use forest_key_management::{
    json::KeyInfoJson,
    KeyInfo,
    Key,
};
use fvm_shared::{
    crypto::signature::SignatureType,
    address::Address,
    sector::{RegisteredSealProof, SectorSize, RegisteredPoStProof},
    version::NetworkVersion,
    econ::TokenAmount,
};
use libp2p::{
    identity::{ed25519, Keypair},
    PeerId,
};
use thiserror::Error;
use figlet_rs::FIGfont;
use anyhow::{anyhow, Error as AnyhowError};
use std::io::{self, Write};
use terminal_menu::{menu, label, button, run, mut_menu};
use crossterm::style::Color;
use log::{info, error};

use wallet;
use miner::{Miner, CreateMinerReturn};
use rpc::RpcEndpoint;
use state::wait_msg;
use logger;
use send::send;
use hex::FromHexError;

#[derive(PartialEq)]
enum YesNo {
    Yes,
    No,
}

impl FromStr for YesNo {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(Self::Yes),
            "no" => Ok(Self::No),
            _ => Ok(Self::No),
        }
    }
}

#[derive(Debug)]
enum AccountType {
    Secp256k1,
    BLS,
}

impl FromStr for AccountType {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Secp256k1" => Ok(Self::Secp256k1),
            "BLS" => Ok(Self::BLS),
            _ => Ok(Self::Secp256k1),
        }
    }
}

enum MinerAction {
    Create,
    ChangeOwner,
}

impl FromStr for MinerAction {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Create" => Ok(Self::Create),
            "ChangeOwner" => Ok(Self::ChangeOwner),
            _ => Ok(Self::Create),
        }
    }
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("io call error")]
    IOCallError(#[from] std::io::Error),
    #[error("parse json error")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("parse hex error")]
    ParseHexError(#[from] FromHexError),
    #[error("common error")]
    CommonError(#[from] AnyhowError),
    #[error("parse url error")]
    ParseUrlError(#[from] url::ParseError),
    #[error("send call error")]
    SendCallError(#[from] send::SendError),
    #[error("miner call error")]
    MinerCallError(#[from] miner::MinerError),
    #[error("state call error")]
    StateCallError(#[from] state::StateError),
}

#[derive(Debug, Subcommand, Clone)]
pub enum Cmd {
    Run {},
}

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

impl Cli {
    pub fn parse(self) -> Result<Self, CliError> {
        Ok(self)
    }

    fn print_banner() {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("FIL Peggy");
        assert!(figure.is_some());
        println!("{}", figure.unwrap());
    }

    pub async fn run(&mut self) -> Result<(), CliError> {
        Self::print_banner();
        Runner::new().run_main().await
    }
}

struct Runner {
    owner: Address,
    owner_key: Option<Key>,
    owner_key_info: Option<KeyInfo>,
    encoded_owner_key: String,

    worker: Address,
    worker_key: Option<Key>,
    worker_key_info: Option<KeyInfo>,
    encoded_worker_key: String,

    fund: Address,
    fund_key_info: Option<KeyInfo>,
    encoded_fund_key: String,

    window_post_proof_type: Option<RegisteredPoStProof>,
    miner_keypair: Option<Keypair>,
    miner_peer_id: Option<PeerId>,
    miner_id_address: Address,
    miner_robust_address: Address,

    rpc_host: String,
    rpc_bearer_token: String,
    rpc: Option<RpcEndpoint>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            owner: Address::default(),
            owner_key: None,
            owner_key_info: None,
            encoded_owner_key: String::default(),

            worker: Address::default(),
            worker_key: None,
            worker_key_info: None,
            encoded_worker_key: String::default(),

            fund: Address::default(),
            fund_key_info: None,
            encoded_fund_key: String::default(),

            window_post_proof_type: None,
            miner_keypair: None,
            miner_peer_id: None,
            miner_id_address: Address::default(),
            miner_robust_address: Address::default(),

            rpc_host: String::default(),
            rpc_bearer_token: String::default(),
            rpc: None,
        }
    }

    async fn run_main(&mut self) -> Result<(), CliError> {
        self.prepare_fund_account()?;
        self.account_handler()?;
        self.prepare_rpc_endpoint()?;
        self.miner_handler().await?;
        self.print_myself()?;
        Ok(())
    }

    fn account_handler(&mut self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no("Would you like to use exist account ?")?;
        match yes_no {
            YesNo::No => {
                self.generate_account()?;
            },
            YesNo::Yes => {
                self.fill_old_account()?;
            },
        }

        Ok(())
    }

    fn prepare_rpc_endpoint(&mut self) -> Result<(), CliError> {
        print!("> {}{}", "Rpc host to lotus".green(), " (e.g. http://localhost:1234/rpc/v0): ".yellow());
        io::stdout().flush().unwrap();

        let mut rpc_host: String = String::default();
        match scanf!("{}", rpc_host) {
            Ok(_) => {
                self.rpc_host = rpc_host.clone();
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        println!("> {}{}", "Lotus bearer token".green(),
            " (e.g. eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIiwid3JpdGUiLCJzaWduIiwiYWRtaW4iXX0.T-IbxWiqPOCak-ZBjXDbDkCBAGGMrPbQvfQTUxtIF10): ".yellow());
        println!("{}", "    Could be ignored if proxy process token for lotus api request!".yellow());

        print!(">   ");
        io::stdout().flush().unwrap();

        let mut bearer_token: String = String::default();
        match scanf!("{}", bearer_token) {
            Ok(_) => {
                self.rpc_bearer_token = bearer_token.clone();
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        self.rpc = Some(RpcEndpoint::new(rpc_host, bearer_token)?);

        Ok(())
    }

    fn prepare_fund_account(&mut self) -> Result<(), CliError> {
        print!("> {}", "Fund address: ".green());
        io::stdout().flush().unwrap();

        let mut fund: Address = Address::default();
        match scanf!("{}", fund) {
            Ok(_) => {
                self.fund = fund;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        print!("> {}", "Fund private key: ".green());
        io::stdout().flush().unwrap();

        let mut key: String = String::default();
        match scanf!("{}", key) {
            Ok(_) => {
                self.encoded_fund_key = key;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        let key_info = hex::decode(&self.encoded_fund_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        self.fund_key_info = Some(KeyInfo::from(key_info));

        Ok(())
    }

    fn fill_old_account(&mut self) -> Result<(), CliError> {
        print!("> {}", "Owner address: ".green());
        io::stdout().flush().unwrap();

        let mut owner: Address = Address::default();
        match scanf!("{}", owner) {
            Ok(_) => {
                self.owner = owner;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        print!("> {}", "Owner private key: ".green());
        io::stdout().flush().unwrap();

        let mut key: String = String::default();
        match scanf!("{}", key) {
            Ok(_) => {
                self.encoded_owner_key = key;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        let key_info = hex::decode(&self.encoded_owner_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        self.owner_key_info = Some(KeyInfo::from(key_info));

        print!("> {}", "Worker address: ".green());
        io::stdout().flush().unwrap();

        let mut worker: Address = Address::default();
        match scanf!("{}", worker) {
            Ok(_) => {
                self.worker = worker;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        print!("> {}", "Worker private key: ".green());
        io::stdout().flush().unwrap();

        let mut key: String = String::default();
        match scanf!("{}", key) {
            Ok(_) => {
                self.encoded_worker_key = key;
            },
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }

        let key_info = hex::decode(&self.encoded_worker_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        self.worker_key_info = Some(KeyInfo::from(key_info));

        Ok(())
    }

    fn generate_account(&mut self) -> Result<(), CliError> {
        let menu = menu(vec![
            label("> Select owner account signature type:").colorize(Color::Green),
            button("Secp256k1"),
            button("BLS")
        ]);
        run(&menu);

        let menu = mut_menu(&menu);
        let account_type = menu.selected_item_name();
        let account_type = match AccountType::from_str(account_type) {
            Ok(AccountType::Secp256k1) => SignatureType::Secp256k1,
            Ok(AccountType::BLS) => SignatureType::BLS,
            Err(_) => SignatureType::Secp256k1,
        };

        let (address, encoded_key, key, key_info_json) = wallet::create_wallet(account_type);
        self.owner = address;
        self.encoded_owner_key = encoded_key.clone();
        self.owner_key = Some(key.clone());
        self.owner_key_info = Some(KeyInfo::from(key_info_json.clone()));

        let yes_no = Runner::yes_no("Use different worker account from owner ?")?;
        if yes_no != YesNo::Yes {
            self.worker = address;
            self.encoded_worker_key = encoded_key;
            self.worker_key = Some(key);
            self.worker_key_info = Some(KeyInfo::from(key_info_json));
            return Ok(());
        }

        let (address, encoded_key, key, key_info_json) = wallet::create_wallet(account_type);
        self.worker = address;
        self.encoded_worker_key = encoded_key;
        self.worker_key = Some(key);
        self.worker_key_info = Some(KeyInfo::from(key_info_json));

        Ok(())
    }

    fn yes_no(s: &str) -> Result<YesNo, CliError> {
        print!("> {}{}", s.green(), " (yes | no): ".yellow());
        io::stdout().flush().unwrap();

        let mut yes_no = YesNo::Yes;
        match scanf!("{}", yes_no) {
            Ok(_) => Ok(yes_no),
            Err(err) => {
                return Err(CliError::IOCallError(err));
            },
        }
    }

    fn print_myself(&self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no("Would you like to display private key?")?;

        println!("> {}", "Cli running information:".blue());
        println!("  > {}{}", "Owner Address:".green(), format!(" {}", self.owner));
        if yes_no == YesNo::Yes {
            println!("  > {}{}", "Owner Private Key:".green(), format!(" {}", self.encoded_owner_key));
        }
        println!("  > {}{}", "Worker Address:".green(), format!(" {}", self.worker));
        if yes_no == YesNo::Yes {
            println!("  > {}{}", "Worker Private Key:".green(), format!(" {}", self.encoded_worker_key));
        }
        println!("  > {}{}", "Fund Address:".green(), format!(" {}", self.fund));
        if yes_no == YesNo::Yes {
            println!("  > {}{}", "Fund Private Key:".green(), format!(" {}", self.encoded_fund_key));
        }

        println!("  > {}{}", "Miner PoSt Proof:".green(), format!(" {:?}", self.window_post_proof_type));
        println!("  > {}{}", "Miner Peer ID:".green(), format!(" {:?}", self.miner_peer_id));
        println!("  > {}{}", "Miner ID Address:".green(), format!(" {}", self.miner_id_address));
        println!("  > {}{}", "Miner Robust Address:".green(), format!(" {}", self.miner_robust_address));

        println!("  > {}{}", "Rpc Host:".green(), format!(" {}", self.rpc_host));
        println!("  > {}{}", "Rpc Bearer Token:".green(), format!(" {}", self.rpc_bearer_token));

        Ok(())
    }

    async fn miner_handler(&mut self) -> Result<(), CliError> {
        let menu = menu(vec![
            label("> Select miner action:").colorize(Color::Green),
            button("Create"),
            button("ChangeOwner")
        ]);
        run(&menu);

        let menu = mut_menu(&menu);
        let action = menu.selected_item_name();
        match MinerAction::from_str(action) {
            Ok(MinerAction::Create) => self.create_miner().await,
            Ok(MinerAction::ChangeOwner) => self.change_owner(),
            Err(err) => Err(CliError::CommonError(err)),
        }
    }

    async fn create_miner(&mut self) -> Result<(), CliError> {
        self.print_myself()?;

        let yes_no = Runner::yes_no("Would you like to create miner with above ^ information?")?;
        if yes_no == YesNo::No {
            return Ok(());
        }

        let menu = menu(vec![
            label("> Select miner's sector size:").colorize(Color::Green),
            button("32GiB"),
            button("64GiB"),
            button("2KiB")
        ]);
        run(&menu);

        let menu = mut_menu(&menu);
        let sector_size = menu.selected_item_name();

        let sector_size =  match sector_size {
            "32GiB" => SectorSize::_32GiB,
            "64GiB" => SectorSize::_64GiB,
            "2KiB" => SectorSize::_2KiB,
            _ => SectorSize::_32GiB,
        };

        let seal_proof = RegisteredSealProof::from_sector_size(sector_size, NetworkVersion::V17);
        match seal_proof.registered_window_post_proof() {
            Ok(proof_type) => {
                self.window_post_proof_type = Some(proof_type);
            },
            Err(err) => {
                return Err(CliError::CommonError(anyhow!("{}", err)));
            },
        }

        let gen_keypair = ed25519::Keypair::generate();
        let net_keypair = Keypair::Ed25519(gen_keypair);
        self.miner_keypair = Some(net_keypair.clone());
        self.miner_peer_id = Some(PeerId::from(net_keypair.public()));

        let rpc_cli: RpcEndpoint;
        match &self.rpc {
            Some(rpc) => {
                rpc_cli = rpc.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid rpc")));
            },
        }

        let fund_key_info: KeyInfo;
        match &self.fund_key_info {
            Some(key_info) => {
                fund_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid fund key info")));
            }
        }

        info!("{}", "> Fund owner address".yellow());
        let _ = send(
            rpc_cli.clone(),
            self.fund,
            fund_key_info.clone(),
            self.owner,
            TokenAmount::from_nano(100_000_000),
        ).await?;

        info!("{}", "> Fund worker address".yellow());
        let _ = send(
            rpc_cli.clone(),
            self.fund,
            fund_key_info.clone(),
            self.worker,
            TokenAmount::from_nano(100_000_000),
        ).await?;

        let owner_key_info: KeyInfo;
        match &self.owner_key_info {
            Some(key_info) => {
                owner_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid owner key info")));
            }
        }

        self.print_myself()?;

        let miner = Miner {
            owner: self.owner,
            owner_key_info: owner_key_info.clone(),
            worker: self.worker,
            window_post_proof_type: self.window_post_proof_type.ok_or(anyhow!("invalid proof type"))?,
            peer_id: self.miner_peer_id.ok_or(anyhow!("invalid peer id"))?,
            rpc: rpc_cli.clone(),
            miner_id: None,
            multiaddrs: None,
        };

        info!("{}", "> Create miner".yellow());
        let res = match miner.create_miner().await {
            Ok(res) => {
                res
            },
            Err(err) => {
                error!("{}", format!(">   Create miner fail: {}", err).red());
                return Err(CliError::MinerCallError(err));
            },
        };

        info!("{}", "> Wait create miner".yellow());
        let ret = wait_msg::<CreateMinerReturn>(
            rpc_cli.clone(),
            res.clone(),
        ).await?;
        self.miner_id_address = ret.id_address;
        self.miner_robust_address = ret.robust_address;

        Ok(())
    }

    fn change_owner(&mut self) -> Result<(), CliError> {
        Ok(())
    }
}

#[derive(Debug)]
enum MenuItem {
    Actor,
}

impl FromStr for MenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            3 => Ok(Self::Actor),
            _ => Ok(Self::Actor),
        }
    }
}

fn select_menu() -> Result<MenuItem, String> {
    println!("{}", "Action you want:".green());
    println!("{}{}", "  3".green(), ". Actor".blue());

    let mut action = MenuItem::Actor;
    match scanf!("{}", action) {
        Ok(_) => {
            Ok(action)
        },
        Err(err) => Err(err.to_string()),
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

pub async fn cli_main() {
    logger::initialize();

    loop {
        let menu = select_menu().unwrap();
        match menu {
            MenuItem::Actor => actor_handler(),
        }
    }
}
