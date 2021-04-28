use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(name = "Ena-Code", about = "Um simples alternador de profile para Visual Studio Code\n\nAinda em alfa.")]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct LaunchOptions {
    #[clap(short, long, about = "Habilita o modo verbose para depuração.")]
    verbose: bool,
    #[clap(default_value = "Default", about = "Nome do profile")]
    profile: String,
    #[clap(default_value = ".", about = "Pasta para usar de workflow.")]
    path: String,
    #[clap(short, long, about = "Muda o derive do profile 'Default' para outro.")]
    base_derive: Option<String>
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
