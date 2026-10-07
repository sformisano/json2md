use std::path::PathBuf;

use clap::{Args as ClapArgs, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "json2md", version, about = "Validate JSON and render Markdown")]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Validate JSON with a schema and render a Markdown template.
    Render(RenderArgs),
}

#[derive(ClapArgs)]
pub(crate) struct RenderArgs {
    /// JSON input file.
    #[arg(value_name = "INPUT")]
    pub(crate) input: PathBuf,

    /// JSON Schema file.
    #[arg(long, value_name = "SCHEMA")]
    pub(crate) schema: PathBuf,

    /// MiniJinja template file.
    #[arg(long, value_name = "TEMPLATE")]
    pub(crate) template: PathBuf,

    /// Write Markdown to this file instead of stdout.
    #[arg(long, value_name = "PATH")]
    pub(crate) output: Option<PathBuf>,
}
