use std::{
    any::type_name_of_val,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use crate::{cli::RunConfig, error::Error};
use anyhow::Context;

pub trait Problem<'a> {
    type Input: Debug;
    type Answer1: Display;
    type Answer2: Display;

    fn init(config: &'a RunConfig) -> Self
    where
        Self: Sized + 'a;

    fn path(&self) -> Result<PathBuf, Error> {
        let type_path = type_name_of_val(self);
        let day = type_path
            .split("::")
            .nth(1)
            .with_context(|| format!("Unexpected type path to '{type_path}'"))?;

        Ok(format!("./src/{day}/input.txt").into())
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
}
