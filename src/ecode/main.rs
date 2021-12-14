mod launcher;
use ecode_core::arguments::LaunchOptions;
use ecode_core::configs::Config;

// extern crate fs_extra;

fn main() {
    let args = LaunchOptions::build();
    let config = Config::get_config(args.verbose);
    launcher::launch(&args, &config);
}
