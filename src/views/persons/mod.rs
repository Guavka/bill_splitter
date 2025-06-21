use crate::store::persons_store::use_person_store;
use crate::views::models::menu::{ActionMenuItem, Menu, SubmenuMenuItem};
use crate::views::persons::persons_list_menu::persons_list_menu;

mod add_people_menu;
mod edit_person_menu;
mod persons_list_menu;
mod remove_person_menu;
mod select_person_menu;

pub fn init_persons_views() -> Menu<'static> {
    let mut person_menu = Menu::new("Меню пользователей", "Назад");

    let mut persons_list_menu2 = Menu::new("Выберите пользователя", "Назад");
    let items_vec = {
        let store = use_person_store();
        store.get_persons_short()
    };
    for item in items_vec {
        persons_list_menu2.add_item(ActionMenuItem {
            title: format!("{}", item),
            action: persons_list_menu,
        });
    }

    person_menu.add_item(SubmenuMenuItem {
        title: "Просмотреть список",
        menu: persons_list_menu2,
    });

    let add_person = Menu::new("Добавление пользователя", "Назад");
    person_menu.add_item(SubmenuMenuItem {
        title: "Добавить",
        menu: add_person,
    });

    person_menu
}
