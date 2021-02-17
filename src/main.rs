mod manager;
use std::process;

use manager::Manager;
mod args;
use clap::Clap;

fn main() {
    let args = args::Args::parse();
    if let Some(shell) = args.completion {
        shell.generate();
        process::exit(0);
    }
    let manager = Manager::new(args);
    println!("{}", manager.run().unwrap());
}