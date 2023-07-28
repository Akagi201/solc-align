use std::path::PathBuf;

use clap::Parser;

#[derive(Clone, Parser, Debug)]
pub struct Cli {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
}
