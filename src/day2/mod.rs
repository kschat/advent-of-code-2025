use std::{
    fmt::{Debug, Display},
    path::Path,
};

use anyhow::Context;

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day2<'a> {
    config: &'a RunConfig,
}

impl<'a> Problem<'a> for Day2<'a> {
    type Input = Vec<ProductId>;
    type Answer1 = usize;
    type Answer2 = usize;

    fn init(config: &'a RunConfig) -> Self
    where
        Self: Sized,
    {
        Self { config }
    }

    fn parse(&self, content: &str, _path: &Path) -> Result<Self::Input, Error> {
        content
            .split('\n')
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .flat_map(|line| line.split(','))
            .map(ProductId::parse)
            .flat_map(|result| match result {
                Ok(vec) => vec.into_iter().map(Ok).collect(),
                Err(error) => vec![Err(error)],
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let result = input
            .iter()
            .filter_map(|id| {
                let (start, end) = id.split()?;

                let invalid = start == end;
                if invalid && self.config.verbose {
                    println!("ID {id}, start {start}, end {end}");
                }

                if !invalid {
                    return None;
                }

                Some(id.as_usize())
            })
            .sum();

        Ok(result)
    }

    fn part2(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let result = input
            .iter()
            .filter_map(|id| {
                id.pivot_points().find_map(|(pivot, (pattern, rest))| {
                    let mut rest = rest;

                    if self.config.verbose {
                        println!("\nPattern {pattern}, rest {rest}");
                    }

                    while rest.len() >= pivot {
                        let (segment, next) = rest.split_at(pivot);
                        rest = next;

                        if self.config.verbose {
                            println!("Checking {} against {pattern}", segment);
                        }

                        if segment != pattern {
                            if self.config.verbose {
                                println!("ID {id} is valid");
                            }

                            return None;
                        }
                    }

                    if self.config.verbose {
                        println!("ID {id} is invalid");
                    }

                    Some(id.as_usize())
                })
            })
            .sum();

        Ok(result)
    }
}

pub struct ProductId(usize, String);

impl ProductId {
    pub fn parse(value: &str) -> Result<Vec<Self>, Error> {
        let mut range = value.split('-');
        let first = range
            .next()
            .with_context(|| "Invalid product ID range, missing first ID")?
            .parse::<usize>()
            .with_context(|| "Invalid product ID given in range")?;

        let second = range
            .next()
            .with_context(|| "Invalid product ID range, missing second ID")?
            .parse::<usize>()
            .with_context(|| "Invalid product ID given in range")?;

        Ok((first..=second)
            .map(|v| ProductId(v, v.to_string()))
            .collect())
    }

    pub fn split(&self) -> Option<(&str, &str)> {
        let (pivot, is_even) = (self.1.len() / 2, self.1.len() % 2 == 0);

        if !is_even {
            return None;
        }

        Some(self.1.split_at(pivot))
    }

    pub fn pivot_points(&self) -> impl Iterator<Item = (usize, (&str, &str))> {
        let length = self.1.len();
        (2..=self.1.len()).filter_map(move |divisor| {
            let (pivot, is_even) = (length / divisor, length % divisor == 0);

            if is_even {
                return Some((pivot, self.1.split_at(pivot)));
            }

            None
        })
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl Debug for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ProductId").field(&self.0).finish()
    }
}

impl Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
