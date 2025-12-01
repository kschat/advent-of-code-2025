use std::{fmt::Display, fs, path::Path};

use crate::{
    cli::{Part, RunConfig},
    error::{Error, ResultExt},
};
use colored::Colorize;

const PADDING: &str = "   ";

pub trait Problem {
    type Input;
    type Answer1: Display;
    type Answer2: Display;

    const PATH: &str;

    fn read_file(&self, path: &Path) -> Result<String, Error> {
        fs::read_to_string(path).map_err(|error| Error::Parse(path.into(), error.to_string()))
    }

    fn parse(&self, _content: &str, _path: &Path) -> Result<Self::Input, Error> {
        Err(Error::Unimplemented)
    }

    fn part1(&self, _input: &Self::Input) -> Result<Self::Answer1, Error> {
        Err(Error::Unimplemented)
    }

    fn part2(&self, _input: &Self::Input) -> Result<Self::Answer2, Error> {
        Err(Error::Unimplemented)
    }

    fn run(&self, config: &RunConfig) -> Result<(), Error> {
        let message = format!("Day {}", config.day).bold();
        println!("🎄 {message}");

        let path = Path::new(Self::PATH);
        let content = self.read_file(path)?;
        let input = self.parse(&content, path)?;

        let answer1 = match config.part {
            Part::One | Part::Both => {
                let answer = self.part1(&input).format();
                format!("{PADDING}Part 1: {answer}\n")
            }
            _ => "".into(),
        };

        let answer2 = match config.part {
            Part::Two | Part::Both => {
                let answer = self.part2(&input).format();
                format!("{PADDING}Part 2: {answer}\n")
            }
            _ => "".into(),
        };

        print!("{answer1}{answer2}");

        Ok(())
    }
}
