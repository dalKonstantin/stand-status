use std::path::PathBuf;

use clap::Parser;

use crate::config::AppConfig;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub config: std::path::PathBuf,
}
