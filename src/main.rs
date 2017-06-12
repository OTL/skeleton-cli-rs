#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate colored;

mod app;

use std::env;
use app::App;
use errors::*;
use log::{LogRecord, LogLevel};
use env_logger::LogBuilder;
use colored::Colorize;

/// Representation of library errors
pub mod errors {
    error_chain!{}
}

fn main() {
    initialize_logger();

    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let cli_app = App;
    cli_app.run_command(&matches)?;

    Ok(())
}

fn initialize_logger() {
    let format = |record: &LogRecord| match record.level() {
        LogLevel::Warn => {
            format!("{} {}", "WARN:".bold().yellow() , record.args().to_string().yellow())
        }
        LogLevel::Error => {
            format!("{} {}", "ERR:".bold().red(), record.args().to_string().red())
        }
        LogLevel::Info => {
            format!("{} {}", "ERR:".bold().green(), record.args().to_string().green())
        }
        LogLevel::Debug => format!("{} {}", "DEBUG:".bold(), record.args().to_string().bold()),
        _ => format!("{}: {}", record.level(), record.args()),
    };
    let mut builder = LogBuilder::new();

    let builder_state = if let Ok(env_log) = env::var("RUST_LOG") {
        builder.format(format).parse(&env_log).init()
    } else {
        builder.format(format).init()
    };

    if let Err(e) = builder_state {
        println!("Could not initialize logger: {}", e);
    }
}