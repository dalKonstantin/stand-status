mod cli;
mod config;
use clap::Parser;

use crate::cli::Args;
use crate::config::AppConfig;

fn main() {
    let args = Args::parse();
    let toml_str = r#"

        [[hosts]]
        name = "att1"
        ip = "192.168.0.1"


        [[hosts]]
        name = "att2"
        ip = "192.168.0.110"
    "#;

    let config: AppConfig = toml::from_str(toml_str).unwrap();
    println!("{:#?}", config);
    print!("{:?}", args);
}
