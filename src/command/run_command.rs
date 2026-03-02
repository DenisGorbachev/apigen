use crate::prompts::get_types_prompt;
use crate::{Context, TryFromAsync};
use clap::{Parser, value_parser};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
pub struct RunCommand {
    /// The source directory with API docs
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    source: PathBuf,

    /// The target directory with API client
    #[arg(short, long, value_parser = value_parser!(PathBuf))]
    target: PathBuf,
}

impl RunCommand {
    pub async fn run(self) -> Result<ExitCode, PrintCommandRunError> {
        // use PrintCommandRunError::*;
        let Self {
            source,
            target,
        } = self;
        let context = Context::try_from_async(&source).await.unwrap();
        let _types = get_types(&context).await;
        create_dir_all(&target).unwrap();
        Ok(ExitCode::SUCCESS)
    }
}

pub async fn get_types(_context: &Context) -> Result<Vec<syn::Item>, ()> {
    let _prompt = get_types_prompt();
    todo!()
}

#[derive(Error, Debug)]
pub enum PrintCommandRunError {}
