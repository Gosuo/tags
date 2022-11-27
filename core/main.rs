use std::path::Path;

use tags::{
    cache::{load_cache, save_cache, TagCache},
    TaggedFile,
};

fn main() {
    let mut tag_file = TaggedFile::new(Path::new("/home/jan/.zshrc"));
    tag_file.add_tag("zsh");
    tag_file.add_tag("config");
    tag_file.add_tag("shell");

    let mut cache = TagCache::empty();
    cache.add_file(tag_file);

    let cache_path = Path::new("/home/jan/tagcachev2.tag");
    save_cache(&cache, cache_path).unwrap();
    let loaded_cache = load_cache(cache_path).unwrap();
    println!("Loaded cache: {loaded_cache:?}");
}
