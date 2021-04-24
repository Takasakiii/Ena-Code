use std::{env, usize};

#[derive(Debug)]
pub struct Args {
    path: String,
    profile: String,
    frags: Vec<String>
}

impl Args {
    pub fn build() -> Self {
        let args: Vec<String> = env::args()
            .collect();


        let path;
        let profile;
        let frags;

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
            frags = args[3..]
                .to_vec();
        } else {
            frags = Vec::new();
        }

        Self {
            path,
            profile,
            frags
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_profile(&self) -> String {
        self.profile.clone()
    }

    pub fn get_flag(&self, index: usize) -> String {
        self.frags[index].clone()
    }

    pub fn has_flag_in_index(&self, index: usize, flag: &str) -> bool {
        if self.frags.len() > index {
            self.frags[index] == flag.to_string()
        } else {
            false
        }
    }

    pub fn exists_flag_in_index(&self, index: usize) -> bool {
        self.frags.len() > index
    }
}
