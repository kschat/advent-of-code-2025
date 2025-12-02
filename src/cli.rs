#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Run the code for a given day and part
    Run(RunConfig),
}

#[derive(Debug, clap::Args, Clone)]
pub struct RunConfig {
    /// Which day to run
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=12))]
    pub day: u8,

    /// The part for the selected day to run
    #[arg(short, long, value_enum, default_value_t)]
    pub part: Part,

    /// Enable verbose output
    #[arg(short, long, default_value_t)]
    pub verbose: bool,

    /// Enable measuring the time it took to complete the problem
    #[arg(short, long, default_value_t)]
    pub metrics: bool,
}

#[derive(Clone, Debug, clap::ValueEnum, Default, PartialEq, Eq)]
pub enum Part {
    #[clap(name = "1")]
    One,

    #[clap(name = "2")]
    Two,

    #[default]
    Both,
}
