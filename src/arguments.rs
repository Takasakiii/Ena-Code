use clap::{AppSettings, Parser};

#[derive(Parser, Debug)]
#[clap(
    name = "Ena-Code",
    about = "A simple profile switcher for Visual Studio Code\n\nStill in alpha."
)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
pub struct LaunchOptions {
    #[clap(short, long, about = "Enables verbose mode for debugging.")]
    pub verbose: bool,
    #[clap(default_value = "Default", about = "Profile name.")]
    pub profile: String,
    #[clap(about = "Workflow folder. ")]
    pub path: Option<String>,
    #[clap(
        short,
        long,
        about = "Changes the 'derive' of the 'Default' profile to another one."
    )]
    pub base_derive: Option<String>,
}

impl LaunchOptions {
    pub fn build() -> Self {
        let args = LaunchOptions::parse();
        if args.verbose {
            println!("{:?}", args);
        }
        args
    }
}
