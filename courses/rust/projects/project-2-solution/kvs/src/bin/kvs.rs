use clap::AppSettings;
use kvs::{Command, KvStore, Result};
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ArgRequiredElseHelp)]
struct Config {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

fn main() -> Result<()> {
    let cfg = Config::from_args();

    let mut kv = KvStore::open(env::current_dir().unwrap()).unwrap();
    match cfg.cmd {
        Some(cmd) => match cmd {
            Command::Get { key } => {
                let value = kv.get(key)?;
                if let Some(v) = value {
                    println!("{}", v);
                } else {
                    println!("Key not found");
                }
            }
            Command::Set { key, value } => {
                kv.set(key, value)?;
            }
            Command::Rm { key } => {
                kv.remove(key)?;
            }
        },
        None => println!("Command not found"),
    }
    Ok(())
}
