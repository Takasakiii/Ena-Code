use std::fs;

use ecode_core::configs::dirs_and_files;

pub fn remove(profile_name: &str) {
    if dirs_and_files::check_profile_exists(profile_name) {
        let confirm = scanln::scanln!(
            "Do you really want to remove the profile {}? [y/N] ",
            profile_name
        );

        if confirm.eq("y") {
            let profile = dirs_and_files::get_profile_path(profile_name);
            fs::remove_dir_all(profile).unwrap();
        }
    } else {
        println!("The profile {} doesn't exist.", profile_name);
    }
}
