use std::fs;

use crate::configs::dirs_and_files;

pub fn get_profiles_list() -> Vec<String> {
    let profile_path = dirs_and_files::get_profiles_folder_path();

    let itens = fs::read_dir(profile_path);

    if let Ok(itens) = itens {
        let mut itens = itens
            .map(|profile| {
                if let Ok(profile) = profile {
                    profile.file_name().to_str().unwrap().to_string()
                } else {
                    "Without permission".to_owned()
                }
            })
            .collect::<Vec<_>>();
        itens.sort();

        itens
    } else {
        vec![]
    }
}
