use crate::components::add_people::AddPersonMenu;
use crate::components::base_component::{Component, EmptyEvents, Menu, Props};
use crate::utils::io::{clear_console, get_number_range};

pub struct MainMenuProps<'a> {
    pub titles: Vec<&'a str>,
}
impl Props for MainMenuProps<'_> {}

pub struct MainMenu<'a> {
    base: Component<MainMenuProps<'a>, EmptyEvents>,
}

impl<'a> MainMenu<'a> {
    pub fn new(props: MainMenuProps<'a>) -> Self {
        Self {
            base: Component {
                props: Some(props),
                events: None,
            },
        }
    }
}

impl Menu for MainMenu<'_> {
    fn show(&self) {
        loop {
            clear_console();

            println!("Добро пожаловать в \"Bill Splitter\"");
            match &self.base.props {
                None => panic!("Props are empty"),
                Some(value) => {
                    for (index, title) in value.titles.iter().enumerate() {
                        println!("{}.{}", index + 1, title);
                    }
                    let selected_index: isize = get_number_range(
                        "Выберите пункт меню",
                        1,
                        value.titles.len() as isize,
                        "Ошибка! Вы ввели не число!",
                        "Ошибка! Вы ввели число вне диапазона!",
                    );

                    match selected_index {
                        1 => AddPersonMenu::new().show(),
                        5 => break,
                        _ => {}
                    }
                }
            }
        }
    }
}
