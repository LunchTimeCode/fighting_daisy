use include_directory::{Dir, File, include_directory};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_directory!("assets");

pub fn read_any_file(name: &str) -> File {
    let path = Path::new(name);
    let file = PROJECT_DIR
        .get_file(path)
        .unwrap_or_else(|| panic!("could not find file this name: {}", path.to_str().unwrap()));
    file.clone()
}
