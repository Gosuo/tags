use std::{
    fs::{read, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use flate2::{bufread::DeflateDecoder, write::DeflateEncoder, Compression};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::TaggedFile;

#[derive(Deserialize, Serialize, Debug)]
pub struct TagCache {
    tagged_files: Vec<TaggedFile>,
}

impl TagCache {
    pub fn empty() -> Self {
        Self {
            tagged_files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: TaggedFile) {
        self.tagged_files.push(file);
    }
}

#[derive(Error, Debug)]
pub enum CacheLoadError {
    #[error("IO error while loading the TagCache {0:?}")]
    IoError(std::io::Error),
    #[error("DeserializationError while loading the TagCache {0:?}")]
    DeserializationError(bincode::Error),
}

impl From<bincode::Error> for CacheLoadError {
    fn from(error: bincode::Error) -> Self {
        Self::DeserializationError(error)
    }
}

impl From<std::io::Error> for CacheLoadError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

#[derive(Error, Debug)]
pub enum CacheSaveError {
    #[error("IO error while saving the TagCache {0:?}")]
    IoError(std::io::Error),
    #[error("Serialization error while saving the TagCache {0:?}")]
    SerializationError(bincode::Error),
}

impl From<bincode::Error> for CacheSaveError {
    fn from(error: bincode::Error) -> Self {
        Self::SerializationError(error)
    }
}

impl From<std::io::Error> for CacheSaveError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

pub fn save_cache(cache: &TagCache, cache_path: &Path) -> Result<(), CacheSaveError> {
    println!("{cache:?}");
    let encoded = bincode::serialize(cache)?;
    println!("encoded: {encoded:?}");

    let cache_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(cache_path)?;

    let mut deflater = DeflateEncoder::new(cache_file, Compression::default());
    deflater.write_all(&encoded).unwrap();
    let _compressed_bytes = deflater.finish()?;

    Ok(())
}

pub fn load_cache(cache_path: &Path) -> Result<TagCache, CacheLoadError> {
    let bytes = read(cache_path)?;
    println!("loaded: {bytes:?}");

    let mut deflater = DeflateDecoder::new(&bytes[..]);
    let mut decompressed_bytes = Vec::new();
    deflater.read_to_end(&mut decompressed_bytes)?;
    println!("encoded: {decompressed_bytes:?}");

    Ok(bincode::deserialize(&decompressed_bytes)?)
}
