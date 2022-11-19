use clap::Parser;
use scanf::scanf;
use std::str::FromStr;

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
    println!("Action you want:");
    println!("  1. Wallet");
    println!("  2. Actor");

    let mut action = MenuItem::Wallet;
    match scanf!("{}", action) {
        Ok(_) => {
            Ok(action)
        },
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    let _ = Args::parse();

    loop {
        let menu = select_menu();
        println!("{:?}", menu.unwrap())
    }
}
