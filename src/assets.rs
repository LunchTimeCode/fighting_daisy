use include_directory::{Dir, File, include_directory};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_directory!("assets");

pub fn read_any_file(name: &str) -> Option<&File<'_>> {
    let path = Path::new(name);
    PROJECT_DIR.get_file(path)
}
