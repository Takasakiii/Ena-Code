use std::{
    env,
    path::{Path, PathBuf},
};

pub trait PathFinder {
    fn get_path<P: AsRef<Path>>(&self, name: P) -> Option<PathBuf>;
}

pub struct EnvPathFinder;

impl PathFinder for EnvPathFinder {
    fn get_path<P: AsRef<Path>>(&self, name: P) -> Option<PathBuf> {
        env::var_os("PATH").and_then(|paths| {
            env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join(&name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
                .next()
        })
    }
}
