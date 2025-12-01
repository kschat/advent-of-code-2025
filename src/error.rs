use colored::{ColoredString, Colorize};
use std::{fmt::Display, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", "Not implemented yet".yellow())]
    Unimplemented,

    #[error(
        "Failed to parse problem input at '{}':\n   ->  {}",
        .0.display().to_string(),
        .1.bold(),
    )]
    Parse(PathBuf, String),

    #[error(
        "Failed to complete problem:\n   ->  {}",
        .0.to_string().bold()
    )]
    Failed(#[from] anyhow::Error),
}

pub trait ResultExt {
    fn format(&self) -> ColoredString;
}

impl<T> ResultExt for Result<T, Error>
where
    T: Display,
{
    fn format(&self) -> ColoredString {
        self.as_ref()
            .map(|v| v.to_string().green())
            .unwrap_or_else(|e| e.to_string().into())
    }
}
