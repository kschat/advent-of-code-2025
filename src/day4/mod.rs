use std::path::Path;

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day4<'a> {
    config: &'a RunConfig,
}

impl<'a> Problem<'a> for Day4<'a> {
    type Input = Vec<Vec<char>>;
    type Answer1 = usize;
    type Answer2 = u64;

    fn init(config: &'a RunConfig) -> Self
    where
        Self: Sized,
    {
        Self { config }
    }

    fn parse(&self, content: &str, _path: &Path) -> Result<Self::Input, Error> {
        let input = content
            .split('\n')
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.chars().collect())
            .collect();

        Ok(input)
    }

    fn part1(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let result = input
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                if self.config.verbose {
                    println!();
                }

                row.iter()
                    .enumerate()
                    .map(move |(column_index, cell)| {
                        if *cell == '.' {
                            if self.config.verbose {
                                print!("{cell}");
                            }

                            return *cell;
                        }

                        let count = check_cell(input, row_index - 1, column_index - 1)
                            + check_cell(input, row_index - 1, column_index)
                            + check_cell(input, row_index - 1, column_index + 1)
                            + check_cell(input, row_index, column_index + 1)
                            + check_cell(input, row_index + 1, column_index + 1)
                            + check_cell(input, row_index + 1, column_index)
                            + check_cell(input, row_index + 1, column_index - 1)
                            + check_cell(input, row_index, column_index - 1);

                        let cell = match count {
                            0..4 => 'x',
                            _ => *cell,
                        };

                        if self.config.verbose {
                            print!("{cell}");
                        }

                        cell
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if self.config.verbose {
            println!();
        }

        let count = result
            .iter()
            .map(|row| row.iter().filter(|c| **c == 'x').count())
            .sum();

        Ok(count)
    }
}

fn check_cell(cells: &[Vec<char>], row: usize, column: usize) -> u32 {
    cells
        .get(row)
        .and_then(|r| r.get(column))
        .map(|c| if *c == '@' { 1 } else { 0 })
        .unwrap_or_default()
}
