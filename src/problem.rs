use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
};

use crate::{
    cli::{Part, RunConfig},
    error::{Error, ResultExt},
    metrics::Metrics,
};
use colored::Colorize;
use humanize_duration::{Truncate, prelude::DurationExt};

const PADDING: &str = "   ";

pub trait Problem {
    type Input: Debug;
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

        let mut metrics = Metrics::start(config.metrics);

        let input = metrics
            .track_parsing(|| self.parse(&content, path))
            .map_err(|error| match error {
                error @ Error::Parse(..) => error,
                error => Error::Parse(path.to_path_buf(), error.to_string()),
            })?;

        if config.verbose {
            println!("{PADDING}Parsing output:");
            println!("{PADDING}{input:#?}");
        }

        let answer1 = match config.part {
            Part::One | Part::Both => {
                let answer = metrics.track_part1(|| self.part1(&input)).format();
                format!("{PADDING}Part 1:  {answer}\n")
            }
            _ => "".into(),
        };

        let answer2 = match config.part {
            Part::Two | Part::Both => {
                let answer = metrics.track_part2(|| self.part2(&input)).format();
                format!("{PADDING}Part 2:  {answer}\n")
            }
            _ => "".into(),
        };

        metrics = metrics.finish();

        print!("{answer1}{answer2}");

        if metrics.enabled {
            println!();
            println!("{}", "🎁 Metrics".bold());

            if let Some(parsing) = metrics.parsing {
                println!(
                    "{PADDING}Parsing: {}",
                    parsing.human(Truncate::Micro).to_string().yellow()
                );
            }

            if let Some(part1) = metrics.part1 {
                println!(
                    "{PADDING}Part 1:  {}",
                    part1.human(Truncate::Micro).to_string().yellow()
                );
            }

            if let Some(part2) = metrics.part2 {
                println!(
                    "{PADDING}Part 2:  {}",
                    part2.human(Truncate::Micro).to_string().yellow()
                );
            }

            println!(
                "{PADDING}Total:   {}",
                metrics.total.human(Truncate::Micro).to_string().yellow()
            );
        }

        Ok(())
    }
}
