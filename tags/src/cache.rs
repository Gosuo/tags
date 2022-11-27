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
pub enum CacheError<'a> {
    #[error("IO error while loading the TagCache from {path} because: {reason}")]
    // IoError(std::io::Error),
    Io {
        path: &'a Path,
        reason: String,
        source: std::io::Error,
    },
    #[error("SerializationError while serializing the TagCache: {reason}")]
    SerializationError { reason: String },
    #[error("DeserializationError while deserializing the TagCache: {reason}")]
    DeserializationError { reason: String },
}

impl From<bincode::Error> for CacheError<'_> {
    fn from(error: bincode::Error) -> Self {
        let reason = error.to_string();
        Self::DeserializationError { reason }
    }
}

// #[derive(Error, Debug)]
// pub enum CacheSaveError {
//     #[error("IO error while saving the TagCache {0:?}")]
//     IoError(std::io::Error),
//     #[error("Serialization error while saving the TagCache {0:?}")]
//     SerializationError(bincode::Error),
// }
//
// impl From<bincode::Error> for CacheSaveError {
//     fn from(error: bincode::Error) -> Self {
//         Self::SerializationError(error)
//     }
// }
//
// impl From<std::io::Error> for CacheSaveError {
//     fn from(error: std::io::Error) -> Self {
//         Self::IoError(error)
//     }
// }

pub fn save_cache<'a>(cache: &TagCache, cache_path: &'a Path) -> Result<(), CacheError<'a>> {
    println!("{cache:?}");
    let encoded = bincode::serialize(cache)?;
    println!("encoded: {encoded:?}");

    let cache_file = match OpenOptions::new().write(true).create(true).open(cache_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(CacheError::Io {
                path: cache_path,
                reason: e.to_string(),
                source: e,
            })
        }
    };

    let mut deflater = DeflateEncoder::new(cache_file, Compression::default());
    deflater.write_all(&encoded).unwrap();
    let _compressed_bytes = match deflater.finish() {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(CacheError::Io {
                path: cache_path,
                reason: e.to_string(),
                source: e,
            })
        }
    };

    Ok(())
}

pub fn load_cache(cache_path: &Path) -> Result<TagCache, CacheError> {
    let bytes = match read(cache_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err(CacheError::Io {
                path: cache_path,
                reason: e.to_string(),
                source: e,
            });
        }
    };
    println!("loaded: {bytes:?}");

    let mut deflater = DeflateDecoder::new(&bytes[..]);
    let mut decompressed_bytes = Vec::new();
    if let Err(e) = deflater.read_to_end(&mut decompressed_bytes) {
        return Err(CacheError::Io {
            path: cache_path,
            reason: e.to_string(),
            source: e,
        });
    };
    println!("encoded: {decompressed_bytes:?}");

    Ok(bincode::deserialize(&decompressed_bytes)?)
}
