use clap::{Parser, Subcommand};
use log::debug;

mod config;
mod errors;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, action)]
    list: bool,

    #[clap(subcommand)]
    action: Subcommand
}

pub enum SshsCommand {
    Connect()
}

pub struct ConnectArgs {
    #[]
}

pub fn run(_config: Args) {
    // if config file is not exists
    debug!("check config file is exists");
    if !config::check() {
        debug!("failed to find config file");
        debug!("initialize config file");
        // create new one
        match config::init() {
            Ok(_) => debug!("config initialize success"),
            Err(err) => panic!("{}", err.to_string()),
        };
    }

    let options = match config::parse() {
        Ok(opt) => opt,
        Err(err) => {
            panic!("{}", err.to_string())
        }
    };

    debug!("{:?}", options);
}
