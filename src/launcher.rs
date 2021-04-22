use crate::{arguments::Args, config::Config};
use std::{path::Path, process::Command};

pub fn launch(args: &Args, config: &Config) {
    let path = Path::new(&config.profiles_folder);   
    let joined_path = path.join(args.get_profile());
    let extension_folder = joined_path.join("extensions");
    let configs_folder = joined_path.join("configs");
    
    let extension_folder = extension_folder.to_str();
    let configs_folder = configs_folder.to_str();

    if extension_folder.is_some() && configs_folder.is_some() {
        let cmd_exec = Command::new(&config.vs_code_path[..])
            .arg(args.get_path())
            .arg("--extensions-dir")
            .arg(extension_folder.unwrap())
            .arg("--user-data-dir")
            .arg(configs_folder.unwrap())
            .output();
        
        match cmd_exec {
            Err(why) => println!("Problema ao iniciar o processo do visual studio code: {:?}", why),
            Ok(out) => println!("{:?}", out)
        }
        
    } else {
        println!("Um problema ao contruir o launch do visual studio code.")
    }
}

