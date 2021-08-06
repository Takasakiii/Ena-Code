use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(
    name = "Ena-Code",
    about = "A simple profile switcher for Visual Studio Code\n\nStill in beta."
)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct LaunchOptions {
    #[clap(subcommand)]
    pub sub: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    #[clap(about = "Incia o Visual Studio Code com um profile.")]
    Run(Run),
    #[clap(about = "Lista os profiles")]
    Ls,
    #[clap(about = "Remove um profile")]
    Rm,
}

#[derive(Clap)]
pub struct Run {
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
        LaunchOptions::parse()
    }
}
