use clap::Parser;
use cli::{Cli, Command};
use colored::Colorize;
use day1::Day1;
use day2::Day2;
use error::Error;
use problem::Problem;

mod cli;
mod error;
mod metrics;
mod problem;

mod day1;
mod day2;

fn run() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Run(config) => match config.day {
            1 => Day1::new(&config).run(&config),
            2 => Day2::new(&config).run(&config),
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
