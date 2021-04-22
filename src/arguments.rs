use std::env;
use clap::Clap;

#[derive(Debug, Clap)]
pub struct NewArgs {
    path: String,
    profile: String,
    cmd: String,
    args: Vec<String>
}


impl NewArgs {
    pub fn build() -> () {

    }
}

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

    #[allow(dead_code)]
    pub fn has_frag(&self, frag: &str) -> bool {
        let item = self.frags
            .iter()
            .find(|e| **e == frag.to_string());
        
        match item {
            Some(_) => true,
            None => false
        }
    }
}