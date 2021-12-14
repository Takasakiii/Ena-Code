use std::fs;

use ecode_core::configs::dirs_and_files;

pub fn list_profiles() {
    let profile_path = dirs_and_files::get_profiles_folder_path();
    let itens = fs::read_dir(profile_path);

    if let Ok(itens) = itens {
        let mut itens = itens
            .map(|profile| {
                if let Ok(profile) = profile {
                    profile.file_name().to_str().unwrap().to_string()
                } else {
                    "Sem Permissão".to_owned()
                }
            })
            .collect::<Vec<_>>();
        itens.sort();

        let print_itens = itens
            .iter()
            .filter(|item| *item != "Sem Permissão" && *item != "Default")
            .fold(String::new(), |acc, new| format!("{}{} ", acc, new));

        println!("{}", print_itens);
    }
}
