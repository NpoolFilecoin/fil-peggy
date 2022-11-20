use clap::Parser;
use scanf::scanf;
use std::str::FromStr;
use colored::Colorize;
use forest_key_management;
use fvm_shared::crypto::signature::SignatureType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
}

#[derive(Debug)]
enum MenuItem {
    Wallet = 1,
    Actor,
}

impl FromStr for MenuItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = s.parse::<i32>()?;
        match item {
            1 => Ok(Self::Wallet),
            2 => Ok(Self::Actor),
            _ => Ok(Self::Wallet),
        }
    }
}

fn select_menu() -> Result<MenuItem, String> {
    println!("{}", "Action you want:".green());
    println!("{}{}", "  1".green(), ". Wallet".blue());
    println!("{}{}", "  2".green(), ". Actor".blue());

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
    println!("{}{}", " Create new wallet: ".yellow(), key.address.to_string());
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
            MenuItem::Actor => actor_handler(),
        }
    }
}
