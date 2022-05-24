use std::{path::PathBuf, str::FromStr};

use anyhow::{bail, Context, Result};
use std::fs::{read, write};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

fn load_png(path: &PathBuf) -> Result<Png> {
    let raw_image = read(path).with_context(|| format!("Failed to read PNG"))?;
    Png::try_from(&raw_image[..])
}

fn write_png(path: &PathBuf, data: Vec<u8>) -> Result<()> {
    write(path, data).with_context(|| format!("Failed to write PNG"))
}

fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();

    Ok(Chunk::new(chunk_type, data))
}

pub fn encode(path: PathBuf, chunk_type: &str, message: &str) -> Result<()> {
    let mut png = load_png(&path)?;
    png.append_chunk(chunk_from_strings(chunk_type, message)?);
    write_png(&path, png.as_bytes())?;
    println!("message encoded");

    Ok(())
}

pub fn decode(path: PathBuf, chunk_type: &str) -> Result<()> {
    let png = load_png(&path)?;
    if let Some(chunk) = png.chunk_by_type(chunk_type) {
        println!("decoded message: {}", chunk.data_as_string()?);
        Ok(())
    } else {
        bail!("chunk {} not found", chunk_type)
    }
}

pub fn remove(path: PathBuf, chunk_type: &str) -> Result<()> {
    let mut png = load_png(&path)?;
    png.remove_chunk(chunk_type)?;
    write_png(&path, png.as_bytes())?;
    println!("chunk removed");

    Ok(())
}

pub fn print(path: PathBuf) -> Result<()> {
    let png = load_png(&path)?;
    println!("{}", png);

    Ok(())
}
