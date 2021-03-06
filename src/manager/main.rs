mod actions;
mod config;
mod list;
mod remove;

use actions::{Commands, LaunchOptions};

fn main() {
    let opt = LaunchOptions::build();
    Commands::handle(&opt);
}
