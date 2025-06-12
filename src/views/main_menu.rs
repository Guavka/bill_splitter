use crate::utils::console_io::clear_console;
use crate::views::persons::person_menu::person_menu;
use crate::views::view_utils::view_utils::get_index_null;

pub fn main_menu() {
    let menu_names: [&str; 3] = ["Пользователи", "Чеки", "Отчеты"];

    loop {
        clear_console();

        let index = get_index_null("Добро пожаловать!", "Выход", &menu_names);
        match index {
            Some(index) => match index {
                1 => person_menu(),
                _ => {}
            },
            None => return,
        }
    }
}
