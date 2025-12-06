use std::path::Path;

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day4<'a> {
    config: &'a RunConfig,
}

impl<'a> Problem<'a> for Day4<'a> {
    type Input = Vec<Vec<char>>;
    type Answer1 = usize;
    type Answer2 = usize;

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
        let mut world = World::new(input, self.config.verbose);
        let count = world.update();

        Ok(count)
    }

    fn part2(&self, input: &Self::Input) -> Result<Self::Answer2, Error> {
        let mut world = World::new(input, self.config.verbose);
        let mut count = world.update();
        let mut total = count;

        while count > 0 {
            count = world.update();
            total += count;
        }

        Ok(total)
    }
}

struct World {
    map: Vec<Vec<char>>,
    count: usize,
    verbose: bool,
}

impl World {
    pub fn new(world: &[Vec<char>], verbose: bool) -> Self {
        Self {
            map: world.to_vec(),
            count: 0,
            verbose,
        }
    }

    pub fn update(&mut self) -> usize {
        let map = self
            .map
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                if self.verbose {
                    println!();
                }

                row.iter()
                    .map(|cell| if *cell == 'x' { '.' } else { *cell })
                    .enumerate()
                    .map(|(column_index, cell)| {
                        if cell == '.' {
                            if self.verbose {
                                print!("{cell}");
                            }

                            return cell;
                        }

                        let count = check_cell(&self.map, row_index - 1, column_index - 1)
                            + check_cell(&self.map, row_index - 1, column_index)
                            + check_cell(&self.map, row_index - 1, column_index + 1)
                            + check_cell(&self.map, row_index, column_index + 1)
                            + check_cell(&self.map, row_index + 1, column_index + 1)
                            + check_cell(&self.map, row_index + 1, column_index)
                            + check_cell(&self.map, row_index + 1, column_index - 1)
                            + check_cell(&self.map, row_index, column_index - 1);

                        let cell = match count {
                            0..4 => 'x',
                            _ => cell,
                        };

                        if self.verbose {
                            print!("{cell}");
                        }

                        cell
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if self.verbose {
            println!();
        }

        let count = map
            .iter()
            .map(|row| row.iter().filter(|c| **c == 'x').count())
            .sum();

        self.count = count;
        self.map = map;

        count
    }
}

fn check_cell(cells: &[Vec<char>], row: usize, column: usize) -> u32 {
    cells
        .get(row)
        .and_then(|r| r.get(column))
        .map(|c| if *c == '@' { 1 } else { 0 })
        .unwrap_or_default()
}
