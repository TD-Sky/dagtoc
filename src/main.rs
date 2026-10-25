use std::process::ExitCode;

use dagtoc::{
    cli::{Cli, SubCommand},
    commands,
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let res = match &cli.command {
        SubCommand::Get(args) => commands::get(args),
        SubCommand::Add(args) => commands::add(args),
        SubCommand::Delete(args) => commands::delete(args),
        SubCommand::Check(args) => commands::check(args),
        SubCommand::Completion(args) => {
            print!("{}", Cli::completion_script(args.shell()));
            Ok(())
        }
    };

    match res {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("dagtoc: {e}");
            ExitCode::FAILURE
        }
    }
}
