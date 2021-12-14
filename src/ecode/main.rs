mod arguments;
mod configs;
mod launcher;
use arguments::LaunchOptions;
use configs::Config;

extern crate fs_extra;

fn main() {
    let args = LaunchOptions::build();
    let config = Config::get_config(args.verbose);
    launcher::launch(&args, &config);
}
