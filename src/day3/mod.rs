use std::path::Path;

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day3;

impl<'a> Problem<'a> for Day3 {
    type Input = Vec<Vec<u64>>;
    type Answer1 = u64;
    type Answer2 = u64;

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
                    .map(|char| char.to_digit(10).unwrap() as u64)
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

                        (*digit, if second < first { first } else { second })
                    },
                );

                (first * 10) + second
            })
            .sum();

        Ok(result)
    }

    fn part2(&self, input: &Self::Input) -> Result<Self::Answer2, Error> {
        let result = input.iter().map(|bank| select_n_batteries(bank, 12)).sum();

        Ok(result)
    }
}

fn select_n_batteries(bank: &[u64], units: usize) -> u64 {
    bank.iter()
        .rev()
        .skip(units)
        .fold(bank[bank.len() - units..].to_vec(), |mut acc, digit| {
            let mut digit = *digit;
            let mut index = 0;

            while index < acc.len() && acc[index] <= digit {
                (digit, acc[index]) = (acc[index], digit);
                index += 1;
            }

            acc
        })
        .iter()
        .rev()
        .enumerate()
        .map(|(exp, digit)| digit * 10_u64.pow(exp as u32))
        .sum()
}
