use crate::utils::console_io::clear_console;
use crate::views::view_utils::view_utils::get_index_null;

// Базовый трейт для пункта меню
pub trait MenuItem {
    fn title(&self) -> &str;
    fn execute(&self);
}

pub struct Menu<'a> {
    title: &'a str,
    items: Vec<Box<dyn MenuItem>>,
    back_text: &'a str,
}

impl<'a> Menu<'a> {
    pub fn new(title: &'a str, back_text: &'a str) -> Self {
        Self {
            title,
            items: Vec::new(),
            back_text,
        }
    }

    pub fn add_item(&mut self, item: impl MenuItem + 'static) {
        self.items.push(Box::new(item));
    }

    pub fn show(&self) {
        loop {
            clear_console();

            let choices: Vec<&str> = self.items.iter().map(|item| item.title()).collect();

            match get_index_null(&self.title, &self.back_text, &choices) {
                Some(index) => self.items[index - 1].execute(),
                None => break,
            }
        }
    }
}

pub struct ActionMenuItem<F> {
    pub title: String,
    pub action: F,
}
impl<F: Fn()> MenuItem for ActionMenuItem<F> {
    fn title(&self) -> &str {
        &self.title
    }
    fn execute(&self) {
        (self.action)();
    }
}

pub struct SubmenuMenuItem<'a> {
    pub title: &'a str,
    pub menu: Menu<'a>,
}

impl MenuItem for SubmenuMenuItem<'_> {
    fn title(&self) -> &str {
        &self.title
    }
    fn execute(&self) {
        self.menu.show();
    }
}

/*
// Пример реализации пункта меню
struct ActionMenuItem<F> {
    title: String,
    action: F,
}
impl<F: Fn()> MenuItem for ActionMenuItem<F> {
    fn title(&self) -> &str {
        &self.title
    }
    fn execute(&self) {
        (self.action)();
    }
}
// Пункт меню, который открывает подменю
struct SubmenuMenuItem {
    title: String,
    menu: Menu<'static>,
}
impl MenuItem for SubmenuMenuItem {
    fn title(&self) -> &str {
        &self.title
    }
    fn execute(&self) {
        self.menu.show();
    }
}
// Пример использования
pub fn setup_main_menu() -> Menu<'static> {
    let mut menu = Menu::new("Добро пожаловать!", "Выход");

    // Меню пользователей
    let mut person_menu = Menu::new("Меню пользователей", "Назад");
    person_menu.add_item(ActionMenuItem {
        title: "Просмотреть список".to_string(),
        action: persons_list_menu,
    });
    person_menu.add_item(ActionMenuItem {
        title: "Добавить".to_string(),
        action: add_people_menu,
    });
    // Добавляем подменю в главное меню
    menu.add_item(SubmenuMenuItem {
        title: "Пользователи".to_string(),
        menu: person_menu,
    });

    // Можно добавить другие меню аналогично
    menu
}
// Использование в main
fn main() {
    let main_menu = setup_main_menu();
    main_menu.show();
}
 */
