use clap::Parser;
use cli::{Cli, Command};
use colored::Colorize;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use error::Error;
use runner::Runner;

mod cli;
mod error;
mod metrics;
mod problem;
mod runner;

mod day1;
mod day2;
mod day3;

fn run() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Run(config) => match config.day {
            1 => Runner::new(&config).run::<Day1>(),
            2 => Runner::new(&config).run::<Day2>(),
            3 => Runner::new(&config).run::<Day3>(),
            _ => Err(Error::Unimplemented),
        },
    }
}

fn main() -> Result<(), Error> {
    if let Err(error) = run() {
        eprintln!("{}: {error}", "error".red());
    }

    Ok(())
}
