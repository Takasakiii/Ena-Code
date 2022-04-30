use ecode_core::actions::get_profiles_list;

pub fn list_profiles() {
    let itens = get_profiles_list();

    let print_itens = itens
        .iter()
        .filter(|item| *item != "Without permission" && *item != "Default")
        .fold(String::new(), |acc, new| format!("{}{} ", acc, new));

    println!("{}", print_itens);
}
