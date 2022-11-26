use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod cache;

#[derive(Deserialize, Serialize, Debug)]
struct Tag(String);

#[derive(Deserialize, Serialize, Debug)]
pub struct TaggedFile {
    path: PathBuf,
    tags: Vec<Tag>,
}

impl TaggedFile {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(Tag(tag.to_string()));
    }
}
