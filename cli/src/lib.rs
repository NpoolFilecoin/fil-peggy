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
    sector::{RegisteredSealProof, SectorSize},
    version::NetworkVersion,
    econ::TokenAmount,
};
use libp2p::{
    identity::{ed25519, Keypair},
    PeerId,
};
use thiserror::Error;
use figlet_rs::FIGfont;
use anyhow;
use std::io::{self, Write};
use terminal_menu::{menu, label, button, run, mut_menu};
use crossterm::style::Color;

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
    type Err = anyhow::Error;

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
    type Err = anyhow::Error;

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
    type Err = anyhow::Error;

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
    CommonError(#[from] anyhow::Error),
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

    #[clap(skip)]
    owner: Address,
    #[clap(skip)]
    owner_key: Option<Key>,
    #[clap(skip)]
    owner_key_info: Option<KeyInfo>,
    #[clap(skip)]
    encoded_owner_key: String,

    #[clap(skip)]
    worker: Address,
    #[clap(skip)]
    worker_key: Option<Key>,
    #[clap(skip)]
    worker_key_info: Option<KeyInfo>,
    #[clap(skip)]
    encoded_worker_key: String,

    #[clap(skip)]
    fund: Address,
    #[clap(skip)]
    fund_key_info: Option<KeyInfo>,
    #[clap(skip)]
    encoded_fund_key: String,
}

impl Cli {
    pub fn parse(self) -> Result<Self, CliError> {
        Ok(self)
    }

    pub fn run(&mut self) -> Result<(), CliError> {
        Self::print_banner();
        self.cli_main()
    }

    fn print_banner() {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("FIL Peggy");
        assert!(figure.is_some());
        println!("{}", figure.unwrap());
    }

    fn cli_main(&mut self) -> Result<(), CliError> {
        self.prepare_fund_account()?;
        self.account_handler()?;
        self.miner_handler()?;
        self.print_myself()?;
        Ok(())
    }

    fn account_handler(&mut self) -> Result<(), CliError> {
        let yes_no = Cli::yes_no("Would you like to use exist account ?")?;
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

        let yes_no = Cli::yes_no("Use different worker account from owner ?")?;
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
        let yes_no = Cli::yes_no("Would you like to display private key?")?;

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

        Ok(())
    }

    fn miner_handler(&mut self) -> Result<(), CliError> {
        let menu = menu(vec![
            label("> Select miner action:").colorize(Color::Green),
            button("Create"),
            button("ChangeOwner")
        ]);
        run(&menu);

        let menu = mut_menu(&menu);
        let action = menu.selected_item_name();
        match MinerAction::from_str(action) {
            Ok(MinerAction::Create) => self.create_miner(),
            Ok(MinerAction::ChangeOwner) => self.change_owner(),
            Err(err) => Err(CliError::CommonError(err)),
        }
    }

    fn create_miner(&mut self) -> Result<(), CliError> {
        Ok(())
    }

    fn change_owner(&mut self) -> Result<(), CliError> {
        Ok(())
    }
}

#[derive(Debug)]
enum MenuItem {
    Miner,
    Actor,
}

impl FromStr for MenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            2 => Ok(Self::Miner),
            3 => Ok(Self::Actor),
            _ => Ok(Self::Miner),
        }
    }
}

fn select_menu() -> Result<MenuItem, String> {
    println!("{}", "Action you want:".green());
    println!("{}{}", "  2".green(), ". Miner".blue());
    println!("{}{}", "  3".green(), ". Actor".blue());

    let mut action = MenuItem::Miner;
    match scanf!("{}", action) {
        Ok(_) => {
            Ok(action)
        },
        Err(err) => Err(err.to_string()),
    }
}

async fn create_miner() {
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

    let mut fund_account: Address = Address::default();
    println!("{}", "Enter fund account address:".green());
    scanf!("{}", fund_account).unwrap();
    println!("{}{}{}", "  You will use ".yellow(), fund_account, " as fund account.".yellow());

    println!("{}", "Enter fund account's key info:".green());
    let mut fund_key_info = String::default();
    scanf!("{}", fund_key_info).unwrap();
    let fund_key_info = hex::decode(&fund_key_info).unwrap();
    let fund_key_info: KeyInfoJson = serde_json::from_slice(&fund_key_info).unwrap();
    println!("{}{:?}", "  Fund KeyInfoJson: ".yellow(), fund_key_info);
    let fund_key_info: KeyInfo = KeyInfo::from(fund_key_info);

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

    let mut rpc_host = String::from("http://localhost:1234/rpc/v0");
    println!("{}{}", "Enter wallet rpc endpoint:".green(), format!("(default {})", rpc_host));
    scanf!("{}", rpc_host).unwrap();
    println!("{}{}{}", "  You will use ".yellow(), rpc_host, " as your rpc host.".yellow());

    let mut bearer_token = String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJyZWFkIiwid3JpdGUiLCJzaWduIiwiYWRtaW4iXX0.T-IbxWiqPOCak-ZBjXDbDkCBAGGMrPbQvfQTUxtIF10");
    println!("{}{}", "Enter rpc bearer token: ".green(), format!("(default {})", bearer_token));
    scanf!("{}", bearer_token).unwrap();
    println!("{}{}{}", "You will use ".yellow(), bearer_token, " as your rpc access token.".yellow());

    let rpc_cli = RpcEndpoint::new(rpc_host, bearer_token).unwrap();

    let _ = send(rpc_cli.clone(), fund_account, fund_key_info.clone(), owner, TokenAmount::from_nano(100_000_000)).await.unwrap();
    let _ = send(rpc_cli.clone(), fund_account, fund_key_info, worker, TokenAmount::from_nano(100_000_000)).await.unwrap();

    let miner = Miner {
        owner: owner,
        owner_key_info: key_info.clone(),
        worker: worker,
        window_post_proof_type: post_proof,
        peer_id: peer_id,
        rpc: rpc_cli.clone(),
        miner_id: None,
        multiaddrs: None,
    };
    let res = miner.create_miner().await.unwrap();
    println!("Create Miner -- {:?}", res.clone());

    let ret = wait_msg::<CreateMinerReturn>(rpc_cli.clone(), res.clone()).await.unwrap();
    println!("Create Miner:");
    println!("  IDAddress: {}", ret.id_address);
    println!("  RobustAddress: {}", ret.robust_address);
}

fn change_owner() {

}

async fn miner_handler() {
    println!("{}", "Miner action you want:".green());
    println!("{}{}", "  1".green(), ". Create".blue());
    println!("{}{}", "  2".green(), ". ChangeOwner".blue());

    let mut action = MinerAction::Create;
    match scanf!("{}", action) {
        Ok(_) => {
            match action {
                MinerAction::Create => {
                    create_miner().await;
                },
                MinerAction::ChangeOwner => {
                    change_owner();
                },
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

pub async fn cli_main() {
    logger::initialize();

    loop {
        let menu = select_menu().unwrap();
        match menu {
            MenuItem::Miner => miner_handler().await,
            MenuItem::Actor => actor_handler(),
        }
    }
}
