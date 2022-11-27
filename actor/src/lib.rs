use git2::{self, Repository};
use thiserror::Error;
use std::path::PathBuf;
use anyhow::Error as AnyhowError;
use std::process::{Command, Stdio};

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("git call error")]
    GitCallError(#[from] git2::Error),
    #[error("io call error")]
    IOCallError(#[from] std::io::Error),
    #[error("common error")]
    CommonError(#[from] AnyhowError),
}

pub fn clone_actor(repo_url: &str, target_path: PathBuf) -> Result<(), ActorError> {
    let _ = Repository::clone(repo_url, target_path)?;
    Ok(())
}

pub fn compile_actor(target_path: PathBuf) -> Result<PathBuf, ActorError> {
    Command::new("cargo")
        .current_dir(target_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("build")
        .arg("--release")
        .output()?;
    Ok(PathBuf::default())
}

pub fn install_actor() {
    println!("{}", " Try deploy actor");
}

pub fn create_actor() {
    println!("{}", " Try create actor");
}

pub fn take_owner() {
    println!("{}", " Try take owner");
}
