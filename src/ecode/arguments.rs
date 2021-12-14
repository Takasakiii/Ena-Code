use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Ena-Code",
    about = "A simple profile switcher for Visual Studio Code\n\nStill in alpha."
)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
pub struct LaunchOptions {
    /// Enables verbose mode for debugging
    #[clap(short, long)]
    pub verbose: bool,

    /// Profile name
    #[clap(default_value = "Default")]
    pub profile: String,

    /// Workflow folder
    pub path: Option<String>,

    /// Changes the 'derive' of the 'Default' profile to another one
    #[clap(short, long)]
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
