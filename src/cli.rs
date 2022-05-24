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
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
    },
    Decode {
        path: PathBuf,
        chunk_type: String,
    },
    Remove {
        path: PathBuf,
        chunk_type: String,
    },
    Print {
        path: PathBuf,
    },
}
