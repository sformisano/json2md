//! Command-line file access and diagnostics for validated Markdown rendering.

mod args;
mod error;
mod output;
mod render;

use std::{io, process::ExitCode};

use clap::Parser;

use args::{Args, Command};

fn main() -> ExitCode {
    let args = Args::parse();
    let result = match args.command {
        Command::Render(args) => render::run(&args),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = error::report(&error, &mut io::stderr().lock());
            ExitCode::FAILURE
        }
    }
}
