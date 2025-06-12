use crate::store::persons_store::use_person_store;
use crate::utils::console_io::clear_console;
use crate::views::persons::select_person_menu::select_person_menu;
use crate::views::view_utils::view_utils::get_index_null;

pub fn persons_list_menu() {
    loop {
        clear_console();

        let array = {
            let store = use_person_store();
            store.get_persons()
        };

        let index = get_index_null("Выберите пользователя", "Назад", &array);
        match index {
            Some(index) => {
                select_person_menu(&array[index - 1]);
            }
            None => return,
        };
    }
}
