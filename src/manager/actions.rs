use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "Ena-Code-Manager",
    about = "Utilitario para gerir os profiles do Ena-Code"
)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
pub struct LaunchOptions {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Gerencia os profiles do Ena-Code
    Profiles {
        #[clap(subcommand)]
        commands: Profiles,
    },
}

#[derive(Subcommand, Debug)]
pub enum Profiles {
    /// Lista os profiles do Ena-Code
    List,
    /// Remove um profile do Ena-Code
    Remove {
        /// Nome do profile
        name: String,
    },
}

impl LaunchOptions {
    pub fn build() -> Self {
        LaunchOptions::parse()
    }
}
