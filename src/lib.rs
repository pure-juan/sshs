use std::process::exit;

use clap::Parser;
use log::{debug, error, info};

use crate::config::{Config, Server};

mod config;
mod ssh;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print list of servers
    #[arg(long, action)]
    list: bool,

    /// action name, currently support `connect`, `init`
    action: Option<String>,

    /// server alias when action is `connect`
    alias: Option<String>,
}

pub fn run(args: Args) {
    // if config file is not exists
    debug!("check config file is exists");
    if !config::check() {
        debug!("failed to find config file");
        debug!("initialize config file");
        // create new one
        match config::init() {
            Ok(_) => debug!("config initialize success"),
            Err(err) => {
                error!("{}", err.to_string());
                exit(1);
            }
        };
    }

    let options = match config::parse() {
        Ok(opt) => opt,
        Err(err) => {
            error!("{}", err.to_string());
            exit(1);
        }
    };

    if args.list {
        print_server_list(options);
        exit(1);
    }

    debug!("{:?}", options);
    if options.servers.len() <= 0 {
        error!("no server definition exists!");
        exit(1);
    }

    if args.action.is_none() {
        error!("action is invalid. (connect)");
        exit(1);
    }
    let action = args.action.unwrap();
    if action == "connect" {
        let alias = args.alias.unwrap();
        debug!("try to find {}.", alias);
        let maybe_server: Option<Server> = find_correct_server(options.servers, alias);

        if maybe_server.is_none() {
            debug!("failed to find server");
            error!("sorry, I can't find that alias in servers");
            exit(1);
        }

        let server = maybe_server.unwrap();
        debug!("found! {:?}", server.clone());

        debug!("initialize ssh agent...");
        let ssh_agent = ssh::init(server);

        debug!("lets connect!");
        ssh_agent.connect()
    } else if action == "init" {
        match config::init() {
            Err(err) => {
                error!("failed to initialize {}", err);
                exit(1);
            }
            Ok(_) => {
                info!("initialized!");
                exit(exitcode::OK)
            }
        }
    } else {
        error!("action is invalid. (connect)");
        exit(1);
    }
}

fn print_server_list(config: Config) {
    for (idx, item) in config.servers.iter().enumerate() {
        println!("{} - alias: {} | {}@{}", idx, item.alias, item.username, item.host)
    }
}

fn find_correct_server(servers: Vec<Server>, alias: String) -> Option<Server> {
    for item in servers {
        if item.alias == alias {
            return Some(item);
        }
    }

    None
}