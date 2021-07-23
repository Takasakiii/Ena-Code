use std::path::PathBuf;

pub trait PathFinder {
    fn get_path(&self) -> &PathBuf;
}
