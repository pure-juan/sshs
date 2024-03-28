use clap::Parser;

use sshs;
use sshs::Args;

fn main() {
    env_logger::init();
    let args = Args::parse();

    sshs::run(args)
}
