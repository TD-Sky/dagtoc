use std::path::PathBuf;

use clap::Args;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub action: Action,

    /// ±count to pages of input/output TOC
    #[arg(
        long,
        short = 'x',
        value_name = "COUNT",
        allow_negative_numbers = true,
        default_value_t = 0
    )]
    pub offset: i32,

    /// Output PDF path
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,

    /// Input PDF path
    pub pdf: PathBuf,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Action {
    /// Delete contents
    #[arg(long, short, requires = "output")]
    pub delete: bool,

    /// Add contents via KDL
    #[arg(long, short, value_name = "TOC", requires = "output")]
    pub add: Option<PathBuf>,

    /// Get contents
    #[arg(long, short)]
    pub get: bool,

    /// Check whether the contents is valid
    /// (all outlines numbered and pages are monotone increasing)
    #[arg(long, short)]
    pub check: bool,
}
