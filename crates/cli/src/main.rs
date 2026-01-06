#![allow(unused_must_use)]

use std::process::exit;

use clap::Parser;
use cli::{Cli, Commands};
// use utils::config::{Config, Field};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
    print!("hello");

    Ok(())
}
