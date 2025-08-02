use crate::components::base_component::Menu;
use crate::store::models::person::Person;
use crate::store::person_store::use_user_store;
use crate::utils::io::{clear_console, get_number_range, get_string_not_empty};

pub struct AddPersonMenu {}
impl AddPersonMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Menu for AddPersonMenu {
    fn show(&self) {
        clear_console();
        loop {
            let name: String = get_string_not_empty(
                "Введите имя:",
                "Ошибка при чтении значения!",
                "Поле не должно быть пустым!",
            );
            let surname: String = get_string_not_empty(
                "Введите фамилию:",
                "Ошибка при чтении значения!",
                "Поле не должно быть пустым!",
            );

            let person = Person::new(&*name, &*surname);
            match person {
                Err(_) => {
                    println!("Возникла ошибка при создании пользователя. Попробуйте позже!");
                    continue;
                }
                Ok(person) => {
                    let mut store = use_user_store();
                    match store.add_person(person) {
                        Err(_) => {
                            println!("Такой пользователь уже существует!");
                            continue;
                        }
                        Ok(_) => {
                            println!("Пользователь {} {} добавлен", name, surname);

                            let selected_index: isize = get_number_range(
                                "Добавить еще?\n1.Да\n2.Нет",
                                1,
                                2,
                                "Ошибка! Вы ввели не число!",
                                "Ошибка! Вы ввели число вне диапазона!",
                            );

                            match selected_index {
                                2 => break,
                                _ => {}
                            }
                        }
                    }
                }
            };
        }
    }
}
