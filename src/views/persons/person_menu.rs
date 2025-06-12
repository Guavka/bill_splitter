use crate::utils::console_io::clear_console;
use crate::views::persons::add_people_menu::add_people_menu;
use crate::views::persons::persons_list_menu::persons_list_menu;
use crate::views::view_utils::view_utils::get_index_null;

pub fn person_menu() {
    let menu_names: [&str; 2] = ["Просмотреть список", "Добавить"];

    loop {
        clear_console();

        let index = get_index_null("Меню пользователей", "Назад", &menu_names);
        match index {
            Some(index) => match index {
                1 => persons_list_menu(),
                2 => add_people_menu(),
                _ => {}
            },
            None => return,
        }
    }
}
