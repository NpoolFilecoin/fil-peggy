use git2::{self, Repository};
use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ActorError {
    #[error("git call error")]
    GitCallError(#[from] git2::Error),
    #[error("io call error")]
    IOCallError(#[from] std::io::Error),
}

pub fn clone_actor(repo_url: &str, target_path: PathBuf) -> Result<(), ActorError> {
    let _ = Repository::clone(repo_url, target_path)?;
    Ok(())
}

pub fn compile_actor() {
    println!("{}", " Try compile actor");
}

pub fn deploy_actor() {
    println!("{}", " Try deploy actor");
}

pub fn create_actor() {
    println!("{}", " Try create actor");
}

pub fn take_owner() {
    println!("{}", " Try take owner");
}
