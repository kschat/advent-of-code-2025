use std::{ops::RangeInclusive, path::Path};

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day5;

impl<'a> Problem<'a> for Day5 {
    type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);
    type Answer1 = usize;
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
            .fold((vec![], vec![]), |(mut ranges, mut ids), line| {
                match line.split_once('-') {
                    Some((first, second)) => {
                        let first = first.parse().unwrap();
                        let second = second.parse().unwrap();
                        ranges.push(first..=second);
                    }
                    None => {
                        ids.push(line.parse().unwrap());
                    }
                }

                (ranges, ids)
            });

        Ok(input)
    }

    fn part1(&self, (ranges, ids): &Self::Input) -> Result<Self::Answer1, Error> {
        let count = ids
            .iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count();

        Ok(count)
    }

    fn part2(&self, (ranges, _): &Self::Input) -> Result<Self::Answer2, Error> {
        let ranges = {
            let mut r = ranges.clone();
            r.sort_by(|a, b| a.start().cmp(b.start()));
            r
        };

        let result = ranges
            .into_iter()
            .fold(vec![], |mut acc, range| {
                let Some(current) = acc.last_mut() else {
                    acc.push(range);
                    return acc;
                };

                let overlap = current.start() <= range.end() && range.start() <= current.end();

                match overlap {
                    true => {
                        let start = current.start().min(range.start());
                        let end = current.end().max(range.end());
                        *current = *start..=*end;
                        acc
                    }
                    false => {
                        acc.push(range);
                        acc
                    }
                }
            })
            .into_iter()
            .map(Iterator::count)
            .sum();

        Ok(result)
    }
}
