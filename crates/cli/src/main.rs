#![allow(unused_must_use)]

use std::process::exit;

use clap::Parser;
use cli::{run};
// use utils::config::{Config, Field};

fn main() -> anyhow::Result<()> {

    print!("hello");
    
    run();

    
    Ok(())
}
