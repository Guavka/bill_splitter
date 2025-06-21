use crate::views::models::menu::{Menu, SubmenuMenuItem};
use crate::views::persons::init_persons_views;

pub mod models;
mod persons;
mod view_utils;

pub fn init_views() -> Menu<'static> {
    let mut menu = Menu::new("Добро пожаловать!", "Выход");
    menu.add_item(SubmenuMenuItem {
        title: "Пользователи",
        menu: init_persons_views(),
    });
    menu.add_item(SubmenuMenuItem {
        title: "Чеки",
        menu: init_persons_views(),
    });
    menu.add_item(SubmenuMenuItem {
        title: "Отчеты",
        menu: init_persons_views(),
    });

    menu
}
