use anyhow::Result;
use clap::Parser;

mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Encode {
            path,
            chunk_type,
            message,
        } => commands::encode(path, chunk_type.as_str(), message.as_str())?,
        cli::Commands::Decode { path, chunk_type } => commands::decode(path, chunk_type.as_str())?,
        cli::Commands::Remove { path, chunk_type } => commands::remove(path, chunk_type.as_str())?,
        cli::Commands::Print { path } => commands::print(path)?,
    };
    Ok(())
}
