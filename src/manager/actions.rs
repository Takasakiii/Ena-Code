use clap::{Parser, Subcommand};

use crate::list;

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

impl Commands {
    pub fn handle(options: &LaunchOptions) {
        match &options.commands {
            Commands::Profiles { commands } => Profiles::handle(commands),
        }
    }
}

impl Profiles {
    fn handle(options: &Profiles) {
        match &options {
            Profiles::List => list::list_profiles(),
            Profiles::Remove { .. } => {}
        }
    }
}

impl LaunchOptions {
    pub fn build() -> Self {
        LaunchOptions::parse()
    }
}
