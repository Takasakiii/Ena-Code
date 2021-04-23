mod arguments;
mod config;
mod launcher;
mod dirs_and_files;

use arguments::Args;
use config::Config;

extern crate fs_extra;

fn main() {
    println!("Projeto ainda esta em desenvolvimento, caso bugs contatar meu github (https://github.com/Takasakiii/Ena-Code), ignorar coisas como logs e config flags sem uso.");
    let args = Args::build();
    let config = Config::get_config();
    launcher::launch(&args, &config);
    println!("{:?}", args);
}
