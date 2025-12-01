use clap::Parser;
use cli::{Cli, Command};
use colored::Colorize;
use day1::Day1;
use error::Error;
use problem::Problem;

mod cli;
mod error;
mod problem;

mod day1;

fn run() -> Result<(), Error> {
    match Cli::parse().command {
        Command::Run(config) => match config.day {
            1 => Day1.run(&config),
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
