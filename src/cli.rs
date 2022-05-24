use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encodes message into PNG file
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
    },
    /// Decodes message from specified chunk of PNG file
    Decode { path: PathBuf, chunk_type: String },
    /// Decodes specified chunk from PNG file
    Remove { path: PathBuf, chunk_type: String },
    /// Prints all chunks of PNG file
    Print { path: PathBuf },
}
