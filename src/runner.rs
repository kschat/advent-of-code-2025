use std::fs;

use colored::Colorize;
use humanize_duration::{Truncate, prelude::DurationExt};

use crate::{
    cli::{Part, RunConfig},
    error::{Error, ResultExt},
    metrics::Metrics,
    problem::Problem,
};

const PADDING: &str = "   ";

pub struct Runner<'a> {
    config: &'a RunConfig,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a RunConfig) -> Self {
        Self { config }
    }

    pub fn run<T>(&self) -> Result<(), Error>
    where
        T: Problem<'a> + 'a,
    {
        let problem = T::init(self.config);
        let message = format!("Day {}", self.config.day).bold();
        println!("🎄 {message}");

        let mut metrics = Metrics::start(self.config.metrics);

        let input = self.read_input(&problem, &mut metrics)?;

        if self.config.verbose {
            println!("{PADDING}Parsing output:");
            println!("{PADDING}{input:#?}");
        }

        let answer1 = match self.config.part {
            Part::One | Part::Both => {
                let answer = metrics.track_part1(|| problem.part1(&input)).format();
                format!("{PADDING}Part 1:  {answer}\n")
            }
            _ => "".into(),
        };

        let answer2 = match self.config.part {
            Part::Two | Part::Both => {
                let answer = metrics.track_part2(|| problem.part2(&input)).format();
                format!("{PADDING}Part 2:  {answer}\n")
            }
            _ => "".into(),
        };

        metrics = metrics.finish();

        print!("{answer1}{answer2}");

        if metrics.enabled {
            self.report_metrics(&metrics);
        }

        Ok(())
    }

    fn read_input<T>(&self, problem: &T, metrics: &mut Metrics) -> Result<T::Input, Error>
    where
        T: Problem<'a>,
    {
        let path = &problem.path()?;
        let content = fs::read_to_string(path)
            .map_err(|error| Error::Parse(path.into(), error.to_string()))?;

        metrics
            .track_parsing(|| problem.parse(&content, path))
            .map_err(|error| match error {
                error @ Error::Parse(..) => error,
                error => Error::Parse(path.to_path_buf(), error.to_string()),
            })
    }

    fn report_metrics(&self, metrics: &Metrics) {
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
}
