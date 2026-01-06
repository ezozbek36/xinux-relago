use clap::{command, Arg, ArgAction, Parser, Subcommand};

use std::io::BufRead;
use subprocess::Exec;


/// CLI interface for server infrastructure
#[derive(Debug, Parser)]
#[command(name = "relago", version)]
#[command(about = "CLI interface for relago", long_about = None)]
#[command(about = "crash reporting tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Exec,
}
