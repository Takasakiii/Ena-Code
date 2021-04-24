use std::{env, usize};

#[derive(Debug)]
pub struct Args {
    path: String,
    profile: String,
    flags: Vec<String>
}

impl Args {
    pub fn build() -> Self {
        let args: Vec<String> = env::args()
            .collect();


        let path;
        let profile;
        let flags;

        if args.len() > 1 {
            profile = args[1].clone();
        } else {
            profile = "Default".into();
        }

        if args.len() > 2 {
            path = args[2].clone();
        } else {
            path = ".".into();
        }

        if args.len() > 3 {
            flags = args[3..]
                .to_vec();
        } else {
            flags = Vec::new();
        }

        Self {
            path,
            profile,
            flags
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_profile(&self) -> String {
        self.profile.clone()
    }

    pub fn get_flag(&self, index: usize) -> String {
        self.flags[index].clone()
    }

    pub fn has_flag_in_index(&self, index: usize, flag: &str) -> bool {
        if self.flags.len() > index {
            self.flags[index] == flag.to_string()
        } else {
            false
        }
    }

    pub fn exists_flag_in_index(&self, index: usize) -> bool {
        self.flags.len() > index
    }
}
