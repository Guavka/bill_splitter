use crate::components::base_component::Menu;
use crate::components::main_menu::{MainMenu, MainMenuProps};

mod components;
mod store;
mod utils;

static IS_DEBUG: bool = true;

fn main() {
    let props = MainMenuProps {
        titles: vec![
            "Добавление участников",
            "Добавление чека",
            "Погашение долгов",
            "Добавление долгов",
            "Выход",
        ],
    };
    MainMenu::new(props).show();
}
