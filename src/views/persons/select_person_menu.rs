use crate::store::models::person::ShortPerson;
use crate::utils::console_io::clear_console;
use crate::views::persons::edit_person_menu::edit_person_menu;
use crate::views::persons::remove_person_menu::remove_person_menu;
use crate::views::view_utils::view_utils::get_index_null;

pub fn select_person_menu(person: &ShortPerson) {
    let menu_names: [&str; 2] = ["Редактировать", "Удалить"];
    let msg = format!("Пользователь `{}`", &person);
    loop {
        clear_console();

        let index = get_index_null(msg.trim(), "Назад", &menu_names);
        match index {
            Some(index) => match index {
                1 => edit_person_menu(person),
                2 => {
                    remove_person_menu(person);
                    return;
                }
                _ => {}
            },
            None => return,
        }
    }
}
