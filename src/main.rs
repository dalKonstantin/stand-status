mod app;
mod cli;
mod config;
mod network;

use crate::app::App;
use crate::cli::Args;
use crate::config::AppConfig;
use clap::Parser;
use std::{fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_content = fs::read_to_string(&args.config)?;
    let config: AppConfig = toml::from_str(&config_content)?;

    let mut terminal = ratatui::init();

    let mut app = App::new(config);
    let mut app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result?;

    Ok(())
}
