use git2::{self, Repository};
use thiserror::Error;
use std::path::PathBuf;
use cargo::{
    core::{Workspace, shell::Shell, compiler::CompileMode},
    util::{
        interning::InternedString,
        config::Config,
    },
    ops::{compile, CompileOptions},
};
use anyhow::{anyhow, Error as AnyhowError};

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
    let home = dirs::home_dir().ok_or(ActorError::CommonError(anyhow!("invalid home")))?;
    let cfg = Config::new(Shell::default(), target_path.clone(), home);

    let workspace = Workspace::new(&target_path.join("Cargo.toml"), &cfg)?;
    let mut compile_opts = CompileOptions::new(&cfg, CompileMode::Build)?;

    compile_opts.build_config.requested_profile = InternedString::new("release");

    compile(&workspace, &compile_opts)?;
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
