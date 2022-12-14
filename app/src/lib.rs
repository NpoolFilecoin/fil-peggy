use clap::{Parser, Subcommand};
use cli::{Cli, CliError};
use thiserror::Error;

#[derive(Debug, Subcommand, Clone)]
pub enum Cmd {
    #[clap(name = "cli")]
    Cli(Cli),
}

#[derive(Debug, Parser)]
#[command(name = "FIL Peggy")]
#[command(author = "The Web3Eye Team <web3.0.eye@gmail.com>")]
pub struct App {
    #[clap(default_value = "2", short, long)]
    pub verbosity: u8,
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("parse args error")]
    ParseArgsError,
    #[error("parse command error")]
    ParseCommandError,
    #[error("run app error")]
    RunAppError,
    #[error("cli execution error {0}")]
    CliExecError(#[from] CliError),
}

impl Cmd {
    pub fn parse(self) -> Result<Self, AppError> {
        match self.clone() {
            Self::Cli(cmd) => {
                let _ = cmd.parse()?;
            }
        }
        Ok(self)
    }

    pub async fn run(self) -> anyhow::Result<(), AppError> {
        match self {
            Self::Cli(mut cmd) => {
                cmd.run().await?;
            }
        }
        Ok(())
    }
}
