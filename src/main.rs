mod arguments;
mod configs;
mod launcher;
use configs::Config;
use arguments::LaunchOptions;

extern crate fs_extra;

fn main() {
    let args = LaunchOptions::build();
    let config = Config::get_config();
    launcher::launch(&args, &config);
}
