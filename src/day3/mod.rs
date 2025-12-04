use std::path::Path;

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day3;

impl<'a> Problem<'a> for Day3 {
    type Input = Vec<Vec<u32>>;
    type Answer1 = u32;
    type Answer2 = usize;

    fn init(_config: &'a RunConfig) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn parse(&self, content: &str, _path: &Path) -> Result<Self::Input, Error> {
        let input = content
            .split('\n')
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        Ok(input)
    }

    fn part1(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let result = input
            .iter()
            .map(|bank| {
                let (first, second) = bank.iter().rev().skip(2).fold(
                    (bank[bank.len() - 2], bank[bank.len() - 1]),
                    |(first, second), digit| {
                        if first > *digit {
                            return (first, second);
                        }

                        if second < first {
                            (*digit, first)
                        } else {
                            (*digit, second)
                        }
                    },
                );

                (first * 10) + second
            })
            .sum();

        Ok(result)
    }
}
