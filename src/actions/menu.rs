use crate::models::person::Person;
use crate::utils::io::{clear_console, get_number_console, get_string_console};
use std::collections::HashSet;

pub fn get_menu_index(menu_items: &[&str]) -> usize {
    loop {
        println!("Добро пожаловать!");
        for (index, value) in menu_items.iter().enumerate() {
            println!("{}.{}", index + 1, value);
        }

        let menu_len = menu_items.len();
        let index: usize = get_number_console(
            "Выберите пункт меню:",
            "Ошибка: введите корректное целое число.",
            Some(Box::new(move |x: usize| -> bool {
                if x < 1 || x > menu_len {
                    println!("Число должно быть от 1 до {}", menu_len);
                    return false;
                }
                true
            })),
        );
        return index;
    }
}

pub fn is_exit(msg: &str) -> bool {
    let is_exit_index: u8 = get_number_console(
        msg,
        "Неверный пункт меню",
        Some(Box::new(|x: u8| -> bool {
            if x < 1 || x > 2 {
                println!("Число должно быть от 1 до 2");
                return false;
            }
            true
        })),
    );
    is_exit_index == 2
}

/// Добавляет новых пользователей в множество.
///
/// Эта функция запрашивает у пользователя имя и фамилию, проверяет,
/// существует ли уже пользователь с такими данными в множестве,
/// и добавляет его, если он уникален.
/// Если пользователь с таким именем и фамилией уже существует,
/// выводится соответствующее сообщение.
///
/// # Параметры
///
/// - `person_set`: Изменяемая ссылка на множество `HashSet<Person>`,
///   в которое будут добавляться новые пользователи.
///
/// # Примечания
///
/// Функция будет продолжать запрашивать данные у пользователя до тех пор,
/// пока он не решит прекратить добавление новых пользователей.
pub fn add_people(person_set: &mut HashSet<Person>) {
    loop {
        let get_string = |msg: &str| -> String {
            get_string_console(
                msg,
                "Возникла проблема при чтении значения",
                Some(Box::new(|x: &String| -> bool {
                    if x.is_empty() {
                        println!("{}", "Поле не должно быть пустым");
                        return false;
                    }
                    true
                })),
            )
        };

        clear_console();
        let name: String = get_string("Введите имя:");
        let surname: String = get_string("Введите фамилию:");

        let is_not_exist = person_set.insert(Person::new(name.clone(), surname.clone()));
        if is_not_exist {
            println!("Пользователь {} {} добавлен", name, surname);
        } else {
            println!("Такой пользователь уже существует");
        }

        if is_exit("Добавить еще?\n1.Да\n2.Нет") {
            break;
        }
    }
}

pub fn take_money() {}
pub fn add_items() {}
pub fn get_money() {}
pub fn reports() {}