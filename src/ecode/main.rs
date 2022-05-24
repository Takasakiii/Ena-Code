mod arguments;
mod launcher;
use arguments::LaunchOptions;
use ecode_core::configs::Config;

fn main() {
    let args = LaunchOptions::build();
    let config = Config::get_config(args.verbose);
    launcher::launch(&args, &config);
}
