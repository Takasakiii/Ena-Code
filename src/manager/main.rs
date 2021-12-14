mod actions;
mod list;

use actions::{Commands, LaunchOptions};

fn main() {
    let opt = LaunchOptions::build();
    Commands::handle(&opt);
}
