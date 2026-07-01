//! Command-line entrypoints for launcher workflows.

use clap::Parser;

mod cli;
mod output;

fn main() {
    match cli::Cli::parse().into_parts() {
        Ok((globals, command)) => output::print_stub(globals, vapor_launcher_core::describe_command(&command)),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    }
}
