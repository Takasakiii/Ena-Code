mod launcher;
mod arguments;
use ecode_core::configs::Config;
use arguments::LaunchOptions;

// extern crate fs_extra;

fn main() {
    let args = LaunchOptions::build();
    let config = Config::get_config(args.verbose);
    launcher::launch(&args, &config);
}
