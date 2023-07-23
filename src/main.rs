mod cli;
use cli::Cli;

mod codec;
mod handlers;
mod toc;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let result = if cli.action.delete {
        handlers::delete(&cli.pdf, &cli.output.unwrap())
    } else if let Some(toc) = cli.action.add {
        handlers::add(&cli.pdf, &cli.output.unwrap(), &toc, cli.offset)
    } else if cli.action.get {
        handlers::get(&cli.pdf, cli.offset)
    } else if cli.action.check {
        handlers::check(&cli.pdf)
    } else {
        unreachable!()
    };

    if let Err(e) = result {
        println!("{e}");
    }
}
