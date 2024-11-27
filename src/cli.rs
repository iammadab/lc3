use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Execute lc3 binary file
    Execute {
        /// Path to binary
        path: String,
    },
    /// Disassemble lc3 binary file
    Disassemble {
        /// Path to binary
        path: String,
    },
}
