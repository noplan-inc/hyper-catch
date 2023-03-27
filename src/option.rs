use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliOptions {
    ///
    #[clap(short, long, value_parser, default_value_os_t = PathBuf::from("./catch.toml"))]
    pub config: PathBuf,

    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Csv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    // chain name
    pub name: String,
    // rpc endpoint
    pub rpc: String,
    // from block height
    pub from: u64,
    // to block height
    pub to: u64,

    // output path ex) ethereum.json, ethereum.csv
    pub output: String,
    // output format
    pub format: OutputFormat,
}
