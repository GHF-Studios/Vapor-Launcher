//! Command-line entrypoints for launcher workflows.

use clap::Parser;

mod cli;
mod output;
mod safety;

fn main() {
    match cli::Cli::parse().into_parts() {
        Ok((globals, command)) => {
            if let Err(error) = output::print_stub(globals, &command) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    }
}
