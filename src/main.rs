//! Weaver CLI tool.

#![allow(clippy::print_stdout)]

use clap::Parser;

use registry::semconv_registry;
use weaver_common::quiet::QuietLogger;
use weaver_common::{ConsoleLogger, Logger};

use crate::cli::{Cli, Commands};

mod cli;
mod registry;

#[cfg(not(tarpaulin_include))]
fn main() {
    let cli = Cli::parse();

    let start = std::time::Instant::now();
    if cli.quiet {
        let log = QuietLogger::new();
        run_command(&cli, log);
    } else {
        let log = ConsoleLogger::new(cli.debug);
        run_command(&cli, log);
    };
    let elapsed = start.elapsed();
    println!("Total execution time: {:?}s", elapsed.as_secs_f64());
}

#[cfg(not(tarpaulin_include))]
fn run_command(cli: &Cli, log: impl Logger + Sync + Clone) {
    match &cli.command {
        Some(Commands::Registry(params)) => {
            semconv_registry(log, params);
        }
        None => {}
    }
}
