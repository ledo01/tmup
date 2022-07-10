mod cli;
mod config;
mod tmux;

use std::{fs::read_dir, path::Path};

use crate::cli::Args;
use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use tmux::Session;

fn file_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

fn find_workspace_file(name: &str) -> Result<String> {
    let workspace_root = Path::new(&dirs::home_dir().unwrap()).join(".tmup");
    let paths = read_dir(workspace_root)?;
    for path in paths.flatten() {
        let file = path.path();
        let stem = file.file_stem().unwrap().to_str().unwrap();
        if stem == name {
            return Ok(path.path().to_str().unwrap().to_string());
        }
    }
    Err(anyhow!("Workspace not found"))
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    let file = match file_exists(&args.file) {
        true => args.file,
        false => find_workspace_file(&args.file)?,
    };

    println!("{}", file);
    let config = Config::from_file(&file).context("Failed to read config from file")?;
    let session = Session::from_config(config);
    session.build()?;

    Ok(())
}
