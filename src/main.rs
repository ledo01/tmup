mod cli;
mod load;
mod tmux;

use crate::cli::Args;
use crate::load::Config;
use anyhow::{Context, Result};
use clap::Parser;
use tmux::Session;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::from_file(args.file).context("Failed to read config from file")?;
    let session = Session::from_config(config);
    session.build()?;

    Ok(())
}
