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
use std::{
    io::{self, Write},
    time::SystemTime,
};
use chrono::{offset::Utc, DateTime};
use terminal_menu::{menu, label, button, run, mut_menu};
use crossterm::style::Color;
use log::{info, error, warn};
use hex::FromHexError;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use std::path::PathBuf;
use forest_json::{
    cid::CidJson,
};
use resolve_path::PathResolveExt;

use wallet;
use miner;
use rpc::RpcEndpoint;
use send::send;
use actor::{
    clone_actor,
    compile_actor,
    install_actor,
    create_actor,
};

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

enum ActorAction {
    Compile,
    Install,
    Create,
    TakeOwner,
}

impl FromStr for ActorAction {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Compile" => Ok(Self::Compile),
            "Install" => Ok(Self::Install),
            "Create" => Ok(Self::Create),
            "TakeOwner" => Ok(Self::TakeOwner),
            _ => Ok(Self::Compile),
        }
    }
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("io call error {0}")]
    IOCallError(#[from] std::io::Error),
    #[error("parse json error {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("parse hex error {0}")]
    ParseHexError(#[from] FromHexError),
    #[error("common error {0}")]
    CommonError(#[from] AnyhowError),
    #[error("parse url error {0}")]
    ParseUrlError(#[from] url::ParseError),
    #[error("send call error {0}")]
    SendCallError(#[from] send::SendError),
    #[error("miner call error {0}")]
    MinerCallError(#[from] miner::MinerError),
    #[error("state call error {0}")]
    StateCallError(#[from] state::StateError),
    #[error("actor call error {0}")]
    ActorCallError(#[from] actor::ActorError),
}

#[derive(Debug, Subcommand, Clone)]
pub enum Cmd {
    CreateMiner {},
    CreateActor {},
    ChangeOwner {},
    CostodyMiner {},
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
        let figure = standard_font.convert("* FIL Peggy");
        assert!(figure.is_some());
        println!("{}", format!("{}", figure.unwrap()).blue().bold());
    }

    pub async fn run(&mut self) -> Result<(), CliError> {
        Self::print_banner();
        match self.cmd {
            Cmd::CreateMiner {} => {
                Runner::new().create_miner_main().await
            },
            Cmd::CreateActor {} => {
                Runner::new().create_actor_main().await
            },
            Cmd::ChangeOwner {} => {
                Runner::new().change_owner_main().await
            },
            Cmd::CostodyMiner {} => {
                Runner::new().take_owner_main().await
            },
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
struct Runner {
    #[serde_as(as = "DisplayFromStr")]
    owner: Address,
    #[serde(skip)]
    owner_key: Option<Key>,
    #[serde(skip)]
    owner_key_info: Option<KeyInfo>,
    encoded_owner_key: String,

    #[serde_as(as = "DisplayFromStr")]
    worker: Address,
    #[serde(skip)]
    worker_key: Option<Key>,
    #[serde(skip)]
    worker_key_info: Option<KeyInfo>,
    encoded_worker_key: String,

    #[serde_as(as = "DisplayFromStr")]
    fund: Address,
    #[serde(skip)]
    fund_key_info: Option<KeyInfo>,
    encoded_fund_key: String,

    window_post_proof_type: Option<RegisteredPoStProof>,
    #[serde(skip)]
    miner_keypair: Option<Keypair>,
    #[serde(skip)]
    miner_peer_id: Option<PeerId>,
    #[serde_as(as = "DisplayFromStr")]
    miner_id_address: Address,
    #[serde_as(as = "DisplayFromStr")]
    miner_robust_address: Address,

    rpc_host: String,
    rpc_bearer_token: String,
    #[serde(skip)]
    rpc: Option<RpcEndpoint>,

    #[serde(default = "String::default")]
    actor_repo_url: String,
    #[serde(default = "String::default")]
    actor_repo_rev: String,
    #[serde(default = "PathBuf::default")]
    actor_path: PathBuf,
    #[serde(default = "PathBuf::default")]
    actor_wasm_path: PathBuf,

    actor_code_id: Option<CidJson>,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "Address::default")]
    actor_id_address: Address,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default = "Address::default")]
    actor_robust_address: Address,
}

impl Runner {
    pub fn new() -> Self {
        match Self::load() {
            Ok(Some(runner)) => {
                return runner;
            },
            Ok(None) => {},
            Err(err) => {
                error!("{}: {}", "> Fail to load exist runner".red(), err);
            },
        }

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

            actor_repo_url: String::default(),
            actor_repo_rev: String::default(),
            actor_path: PathBuf::default(),
            actor_wasm_path: PathBuf::default(),

            actor_code_id: None,
            actor_id_address: Address::default(),
            actor_robust_address: Address::default(),
        }
    }

    fn load() -> Result<Option<Self>, CliError> {
        let yes_no = Runner::yes_no("Would you like to use exist runner?", true)?;
        if yes_no == YesNo::No {
            return Ok(None);
        }

        let mut paths = Vec::new();
        for dir in std::fs::read_dir("output")? {
            let path = dir?.path();
            if path.is_dir() {
                continue
            }
            paths.push(format!("{}", path.display()));
        }

        if paths.len() == 0 {
            return Ok(None);
        }

        let mut menus = Vec::new();
        menus.push(label("> Select saved runner:").colorize(Color::Green));
        for path in paths {
            menus.push(button(path));
        }

        let menu = menu(menus);
        run(&menu);

        let menu = mut_menu(&menu);
        let runner_file = menu.selected_item_name();

        let runner_str = std::fs::read_to_string(runner_file)?;

        let mut runner: Self;
        match serde_json::from_str(&runner_str) {
            Ok(r) => {
                runner = r;
            },
            Err(err) => {
                error!("{}", format!("> Fail parse json: {}", err).red());
                return Err(CliError::ParseJsonError(err));
            },
        }

        runner.print_myself()?;

        runner.rpc = Some(RpcEndpoint::new(runner.clone().rpc_host, runner.clone().rpc_bearer_token)?);

        let key_info = hex::decode(&runner.clone().encoded_owner_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        runner.owner_key_info = Some(KeyInfo::from(key_info));

        let key_info = hex::decode(&runner.clone().encoded_worker_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        runner.worker_key_info = Some(KeyInfo::from(key_info));

        let key_info = hex::decode(&runner.clone().encoded_fund_key)?;
        let key_info: KeyInfoJson = serde_json::from_slice(&key_info)?;
        runner.fund_key_info = Some(KeyInfo::from(key_info));

        Ok(Some(runner))
    }

    async fn create_actor_main(&mut self) -> Result<(), CliError> {
        self.actor_repo_handler().await?;
        self.save_myself()?;
        self.compile_actor()?;
        self.install_actor().await?;
        self.create_actor().await?;
        self.print_myself()?;
        self.save_myself()?;
        Ok(())
    }

    async fn change_owner(&self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no(&format!(
            "{}:\n  {}{}{}{}{}",
            "Would you like to change".bright_green().bold(),
            self.miner_id_address.to_string().bold(),
            "'s owner from ".bright_green(),
            self.owner.to_string().bold(),
            " to ".bright_green(),
            self.actor_id_address.to_string().bold(),
        ), false)?;
        if yes_no == YesNo::No {
            return Ok(());
        }

        let rpc_cli: RpcEndpoint;
        match &self.rpc {
            Some(rpc) => {
                rpc_cli = rpc.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid rpc")));
            },
        }

        let owner_key_info: KeyInfo;
        match &self.owner_key_info {
            Some(key_info) => {
                owner_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid owner key info")));
            }
        }

        match miner::change_owner(
            rpc_cli,
            self.owner,
            owner_key_info,
            self.miner_id_address,
            self.actor_id_address,
        ).await {
            Ok(_) => Ok(()),
            Err(err) => Err(CliError::MinerCallError(err)),
        }
    }

    async fn change_owner_main(&self) -> Result<(), CliError> {
        self.change_owner().await
    }

    async fn take_owner(&self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no(&format!(
            "{}:\n  {}{}{}{}{}",
            "Would you like to take".bright_green().bold(),
            self.miner_id_address.to_string().bold(),
            "'s owner from ".bright_green(),
            self.owner.to_string().bold(),
            " to ".bright_green(),
            self.actor_id_address.to_string().bold(),
        ), false)?;
        if yes_no == YesNo::No {
            return Ok(());
        }

        let rpc_cli: RpcEndpoint;
        match &self.rpc {
            Some(rpc) => {
                rpc_cli = rpc.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid rpc")));
            },
        }

        let owner_key_info: KeyInfo;
        match &self.owner_key_info {
            Some(key_info) => {
                owner_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid owner key info")));
            }
        }

        match actor::take_owner(
            rpc_cli,
            self.owner,
            owner_key_info,
            self.actor_id_address,
            self.miner_id_address,
        ).await {
            Ok(_) => Ok(()),
            Err(err) => Err(CliError::ActorCallError(err)),
        }
    }

    async fn take_owner_main(&self) -> Result<(), CliError> {
        self.take_owner().await
    }

    async fn actor_repo_handler(&mut self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no("Would you like to use exist repository?", true)?;
        if yes_no == YesNo::Yes {
            return Ok(());
        }

        print!("> {}", "Actor git repository: ".green());
        io::stdout().flush().unwrap();

        let mut repo_url = String::default();
        scanf!("{}", repo_url)?;

        print!("> {}", "Actor git revision: ".green());
        io::stdout().flush().unwrap();

        let mut repo_rev = String::from("master");
        scanf!("{}", repo_rev)?;

        print!("> {}", "Clone to: ".green());
        io::stdout().flush().unwrap();

        let mut target_path = PathBuf::default();
        scanf!("{}", target_path)?;

        self.actor_repo_url = repo_url.clone();
        self.actor_repo_rev = repo_rev.clone();
        self.actor_path = target_path.clone().resolve().to_path_buf();

        info!("{}{}{}{}", "> Cloning ...".blue().bold(), repo_url.clone(), " -> ".yellow(), target_path.clone().display());
        clone_actor(&repo_url, &repo_rev, target_path.clone())?;

        Ok(())
    }

    fn compile_actor(&mut self) -> Result<(), CliError> {
        info!("{}{}", "> Compiling ...".blue().bold(), self.actor_path.clone().display());
        self.actor_wasm_path = compile_actor(self.actor_path.clone())?;
        Ok(())
    }

    async fn install_actor(&mut self) -> Result<(), CliError> {
        let rpc_cli: RpcEndpoint;
        match &self.rpc {
            Some(rpc) => {
                rpc_cli = rpc.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid rpc")));
            },
        }

        let owner_key_info: KeyInfo;
        match &self.owner_key_info {
            Some(key_info) => {
                owner_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid owner key info")));
            }
        }

        info!("{}{}", "> Installing ... ".blue().bold(), self.actor_wasm_path.clone().display());
        let (code_cid, installed) = install_actor(
            rpc_cli,
            self.owner,
            owner_key_info.clone(),
            self.actor_wasm_path.clone(),
        ).await?;

        self.actor_code_id = Some(code_cid.clone());
        if !installed {
            warn!("> Actor {:?} may be already installed", code_cid);
        }

        Ok(())
    }

    async fn create_actor(&mut self) -> Result<(), CliError> {
        let rpc_cli: RpcEndpoint;
        match &self.rpc {
            Some(rpc) => {
                rpc_cli = rpc.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid rpc")));
            },
        }

        let owner_key_info: KeyInfo;
        match &self.owner_key_info {
            Some(key_info) => {
                owner_key_info = key_info.clone();
            },
            _ => {
                return Err(CliError::CommonError(anyhow!("invalid owner key info")));
            }
        }

        let actor_code_id: &CidJson;
        match &self.actor_code_id {
            Some(code_id) => {
                actor_code_id = code_id;
            },
            None => {
                return Err(CliError::CommonError(anyhow!("invalid actor code id")));
            },
        }

        info!("{}{:?}", "> Creating ... ".blue().bold(), actor_code_id.clone());
        let (id_address, robust_address) = create_actor(
            rpc_cli,
            self.owner,
            owner_key_info.clone(),
            actor_code_id.clone(),
        ).await?;

        self.actor_id_address = id_address;
        self.actor_robust_address = robust_address;

        Ok(())
    }

    async fn create_miner_main(&mut self) -> Result<(), CliError> {
        self.prepare_fund_account()?;
        self.account_handler()?;
        self.prepare_rpc_endpoint()?;
        self.miner_handler().await?;
        self.print_myself()?;
        self.save_myself()?;
        Ok(())
    }

    fn account_handler(&mut self) -> Result<(), CliError> {
        let yes_no = Runner::yes_no("Would you like to use exist account?", true)?;
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
        let yes_no = Runner::yes_no("Would you like to use exist rpc endpoint?", true)?;
        if yes_no == YesNo::Yes {
            return Ok(());
        }

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
        let yes_no = Runner::yes_no("Would you like to use exist fund account?", true)?;
        if yes_no == YesNo::Yes {
            return Ok(());
        }

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

        let yes_no = Runner::yes_no("Use different worker account from owner?", true)?;
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

    fn yes_no(s: &str, label_color: bool) -> Result<YesNo, CliError> {
        match label_color {
            true => print!("> {}{}", s.bright_green().bold(), " (yes | no): ".yellow()),
            _ => print!("> {}{}", s, " (yes | no): ".yellow()),
        } 
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
        let yes_no = Runner::yes_no("Would you like to display private key?", true)?;

        println!("> {}", "Cli running information:".blue().bold());
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

        println!("  > {}{}", "Actor Repo Url:".green(), format!(" {}", self.actor_repo_url));
        println!("  > {}{}", "Actor Repo Revision:".green(), format!(" {}", self.actor_repo_rev));
        println!("  > {}{}", "Actor Path:".green(), format!(" {}", self.actor_path.display()));
        println!("  > {}{}", "Actor WASM Path:".green(), format!(" {}", self.actor_wasm_path.display()));

        println!("  > {}{}", "Actor Code ID:".green(), format!(" {:?}", self.actor_code_id));
        println!("  > {}{}", "Actor ID Address:".green(), format!(" {}", self.actor_id_address));
        println!("  > {}{}", "Actor Robust Address:".green(), format!(" {}", self.actor_robust_address));

        Ok(())
    }

    fn save_myself(&self) -> Result<(), CliError> {
        match std::fs::create_dir("output") {
            Ok(_) => {},
            Err(err) => {
                if err.kind() != std::io::ErrorKind::AlreadyExists {
                    return Err(CliError::IOCallError(err));
                }
            },
        }

        let my_json = serde_json::to_string_pretty(self)?;
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();

        let filename = format!("output/peggy-{}.json", datetime.format("%d-%m-%Y-%H-%M-%s"));
        std::fs::write(filename.clone(), my_json)?;

        info!("> {}", format!("Result is saved at {}", filename).blue().bold());

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
            Ok(MinerAction::ChangeOwner) => self.change_owner().await,
            Err(err) => Err(CliError::CommonError(err)),
        }
    }

    async fn create_miner(&mut self) -> Result<(), CliError> {
        self.print_myself()?;

        let yes_no = Runner::yes_no("Would you like to create miner with above ^ information?", true)?;
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

        info!("{}", "> Create miner".yellow());
        let (id_address, robust_address) = match miner::create_miner(
            rpc_cli.clone(),
            self.owner,
            owner_key_info.clone(),
            self.worker,
            self.window_post_proof_type.ok_or(anyhow!("invalid proof type"))?,
            self.miner_peer_id.ok_or(anyhow!("invalid peer id"))?,
        ).await {
            Ok(res) => {
                res
            },
            Err(err) => {
                error!("{}", format!(">   Create miner fail: {}", err).red());
                return Err(CliError::MinerCallError(err));
            },
        };

        self.miner_id_address = id_address;
        self.miner_robust_address = robust_address;

        Ok(())
    }
}

