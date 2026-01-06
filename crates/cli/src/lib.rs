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

pub fn init() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("exec").action(ArgAction::Append))
        .get_matches();

    let args = matches
        .get_many::<String>("exec")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let sbcmd = args[1];
    let _ = cmd_exec(sbcmd.clone());

    let owned_words: Vec<&str> = sbcmd.split_whitespace().map(|x| x).collect();

    Ok(())
}

fn cmd_exec(cmd: &str) -> anyhow::Result<()> {
    let cm = Exec::shell(cmd);

    match cm.clone().capture() {
        Ok(x) => {
            //   print!("FOWARDED COMMAND: {}", x.stderr_str());
            // let v = cm.stream_stdout()?;
            let v = cm.stream_stderr()?;
            let mut reader = std::io::BufReader::new(v);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        println!("Line :{}", l);
                    }
                    Err(e) => print!("Error:{}", e),
                }
            }
        }
        Err(e) => {
            print!("{}", e)
        }
    }

    Ok(())
}
