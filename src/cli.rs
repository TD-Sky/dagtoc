use std::path::PathBuf;

use usage_rs::{Args, Subcommands, complete::Shell};

#[derive(usage_rs::Cli)]
#[usage(bin = "dagtoc", version, completion)]
pub struct Cli {
    #[usage(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommands)]
pub enum SubCommand {
    /// Get contents
    #[usage(visible_alias = "g")]
    Get(GetArgs),

    /// Add contents via KDL
    #[usage(visible_alias = "a")]
    Add(AddArgs),

    /// Delete contents
    #[usage(visible_alias = "d")]
    Delete(DeleteArgs),

    /// Check whether the contents is valid
    /// (all outlines numbered and pages are monotone increasing)
    #[usage(visible_alias = "c")]
    Check(CheckArgs),

    /// Print a completion script
    Completion(Completion),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// TOC to be added
    #[usage(long)]
    pub toc: PathBuf,

    /// Input PDF path
    pub pdf: PathBuf,

    /// Output PDF path
    #[usage(long, short)]
    pub output: PathBuf,

    /// ±count to pages of input/output TOC
    #[usage(
        long,
        short = 'x',
        value_name = "COUNT",
        allow_negative_numbers = true,
        default = "0"
    )]
    pub offset: isize,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Input PDF path
    pub pdf: PathBuf,

    /// ±count to pages of input/output TOC
    #[usage(
        long,
        short = 'x',
        value_name = "COUNT",
        allow_negative_numbers = true,
        default = "0"
    )]
    pub offset: isize,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Input PDF path
    pub pdf: PathBuf,

    /// Output PDF path
    #[usage(long, short)]
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Input PDF path
    pub pdf: PathBuf,
}

#[derive(Args)]
pub struct Completion {
    /// Which shell to generate for
    #[usage(long, choices("bash", "zsh", "fish", "nu", "powershell"))]
    shell: String,
}

impl Completion {
    pub fn shell(&self) -> Shell {
        Shell::from_name(&self.shell).expect("verified during parsing cli")
    }
}
