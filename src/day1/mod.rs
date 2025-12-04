use std::{fmt::Display, path::Path};

use anyhow::{Context, anyhow};

use crate::{cli::RunConfig, error::Error, problem::Problem};

pub struct Day1<'a> {
    config: &'a RunConfig,
}

impl<'a> Day1<'a> {
    pub fn new(config: &'a RunConfig) -> Self {
        Self { config }
    }
}

impl<'a> Problem<'a> for Day1<'a> {
    type Input = Vec<Rotation>;
    type Answer1 = u16;
    type Answer2 = u16;

    fn init(config: &'a RunConfig) -> Self
    where
        Self: Sized,
    {
        Self { config }
    }

    fn parse(&self, content: &str, _path: &Path) -> Result<Self::Input, Error> {
        content
            .split('\n')
            .filter(|rotation| !rotation.trim().is_empty())
            .map(Rotation::parse)
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let (count, _) = input
            .iter()
            .fold((0, Dial::new()), |(count, dial), rotation| {
                let dial = dial.rotate(*rotation);
                if self.config.verbose {
                    println!("The dial is rotated {rotation} to point at {dial}");
                }

                match dial.position {
                    0 => (count + 1, dial),
                    _ => (count, dial),
                }
            });

        Ok(count)
    }

    fn part2(&self, input: &Self::Input) -> Result<Self::Answer1, Error> {
        let dial = input.iter().fold(Dial::new(), |dial, rotation| {
            let dial = dial.rotate(*rotation);
            if self.config.verbose {
                println!("The dial is rotated {rotation} to point at {dial}");
            }

            dial
        });

        Ok(dial.odometer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Left => "L",
            Self::Right => "R",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    value: i16,
    direction: Direction,
}

impl Rotation {
    pub fn parse(value: &str) -> Result<Self, Error> {
        let (direction, value) = match value.split_at(1) {
            ("L", value) => (Direction::Left, -Self::parse_value(value)?),
            ("R", value) => (Direction::Right, Self::parse_value(value)?),
            value => {
                return Err(anyhow!("Failed to parse rotation {value:?}").into());
            }
        };

        Ok(Self { value, direction })
    }

    fn parse_value(value: &str) -> Result<i16, Error> {
        let result = value
            .parse::<i16>()
            .with_context(|| format!("Non-i16 value given for rotation: '{value}'"))?;

        Ok(result)
    }

    pub fn normalize(&self) -> (u16, i16) {
        let base = match self.direction {
            Direction::Right => 100,
            Direction::Left => -100,
        };

        ((self.value / base) as u16, self.value % base)
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = self.direction;
        let value = self.value.abs();
        write!(f, "{direction}{value}")
    }
}

struct Dial {
    /// The current position of the dial.
    pub position: u16,

    /// How many times the dial has passed `0`.
    ///
    /// Be sure to get the dial serviced every 10,000 rotations!
    pub odometer: u16,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            position: 50,
            odometer: 0,
        }
    }

    pub fn rotate(self, rotation: Rotation) -> Self {
        let (rotations, normalized) = rotation.normalize();
        let position = self.position.wrapping_add_signed(normalized) % (u16::MAX - 99) % 100;

        if position == 0 {
            return Self {
                position,
                odometer: self.odometer + rotations + 1,
            };
        }

        let rotations = match (self.position, rotation.direction) {
            (1.., Direction::Right) if position <= self.position => rotations + 1,
            (1.., Direction::Left) if position >= self.position => rotations + 1,
            _ => rotations,
        };

        Self {
            position,
            odometer: self.odometer + rotations,
        }
    }
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: {}, odometer: {}",
            self.position, self.odometer
        )
    }
}
