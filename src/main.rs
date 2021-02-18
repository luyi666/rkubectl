mod manager;
mod args;
mod logger;

use std::process;
use log::info;
use manager::Manager;
use clap::Clap;
use logger::Logger;

fn main() {
    // assume Logger::init is Ok
    Logger::init().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    let args = args::Args::parse();
    if let Some(shell) = args.completion {
        shell.generate();
        process::exit(0);
    }
    if let None = args.cmd {
        info!("please provide a command, see details with --help");
        process::exit(0);
    }
    let manager = Manager::new(args);
    let result_message = manager.run();
    // print result message to stdout if successful so that you redirect the message, otherwise print to stderr
    match result_message {
       Ok(message) => println!("{}", message),
       Err(e) => eprintln!("{}", e),
    }
}