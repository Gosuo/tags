use std::{
    fs::{read, File, OpenOptions},
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
}

#[derive(Error, Debug)]
pub enum CacheSaveError {
    #[error("IO error while saving the TagCache {0:?}")]
    IoError(std::io::Error),
    #[error("Serialization error while saving the TagCache {0:?}")]
    SerializationError(bincode::Error),
}

pub fn save_cache(cache: &TagCache, cache_path: &Path) -> Result<(), CacheSaveError> {
    println!("{cache:?}");
    let encoded: Vec<u8> = match bincode::serialize(cache) {
        Ok(bytes) => bytes,
        Err(e) => return Err(CacheSaveError::SerializationError(e)),
    };
    println!("encoded: {encoded:?}");

    // let deflater = DeflateEncoder::new(encoded, Compression::default());
    // let compressed_bytes = match deflater.finish() {
    //     Ok(bytes) => bytes,
    //     Err(e) => return Err(CacheSaveError::IoError(e)),
    // };
    // println!("compressed: {compressed_bytes:?}");

    let mut cache_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(cache_path)
    {
        Ok(f) => f,
        Err(e) => return Err(CacheSaveError::IoError(e)),
    };

    // cache_file.write_all(&compressed_bytes).unwrap();
    cache_file.write_all(&encoded).unwrap();

    Ok(())
}

pub fn load_cache(cache_path: &Path) -> Result<TagCache, CacheLoadError> {
    let bytes = match read(cache_path) {
        Err(io_error) => return Err(CacheLoadError::IoError(io_error)),
        Ok(bytes) => bytes,
    };
    println!("loaded: {bytes:?}");

    // let mut deflater = DeflateDecoder::new(&bytes[..]);
    // let mut decompressed_bytes = Vec::new();
    // if let Err(io_error) = deflater.read_to_end(&mut decompressed_bytes) {
    //     return Err(CacheLoadError::IoError(io_error));
    // }

    // let cache: TagCache = bincode::deserialize(&decompressed_bytes).unwrap();
    let cache: TagCache = bincode::deserialize(&bytes).unwrap();

    Ok(cache)
}
