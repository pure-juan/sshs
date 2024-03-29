use std::process::{Command, Stdio};

use log::debug;

use crate::config::Server;

pub struct Agent {
    server: Server,
}

impl Agent {
    fn new(server: Server) -> Agent {
        Agent {
            server
        }
    }

    pub fn connect(&self) {
        let mut command = Command::new("ssh");
        command.arg(format!("{}@{}", self.server.username, self.server.host).as_str());
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        if self.server.identity.is_some() {
            let identity = Clone::clone(&self.server.identity).unwrap();
            if !identity.is_empty() {
                command.arg(format!("-i {}", identity));
            }
        }

        match command.spawn() {
            Err(e) => {
                debug!("failed to execute ssh agent {}", e.to_string());
            }
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            debug!("successfully connected!");
                        } else {
                            debug!("failed to connect {}", status.to_string());
                        }
                    }
                    Err(err) => {
                        debug!("failed to connect {}", err);
                    }
                };
            }
        };
    }
}

pub fn init(server: Server) -> Agent {
    let agent = Agent::new(server);

    agent
}