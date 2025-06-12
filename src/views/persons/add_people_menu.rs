use crate::store::models::person::{EPersonError, Person, PersonSettings};
use crate::store::persons_store::use_person_store;
use crate::utils::console_io::{clear_console, get_console};
use crate::views::view_utils::view_utils::get_index_null;

/// Добавляет нового человека в карту и сохраняет данные в файл
///
/// # Параметры
/// - `person_map`: изменяемая ссылка на HashMap, хранящий людей, где ключом является уникальный идентификатор,
/// а значением - структура `Person`.
///
/// # Описание
/// Функция запрашивает у пользователя имя и фамилию, создает нового человека и добавляет его в `person_map`.
/// Если пользователь с таким идентификатором уже существует, выводится сообщение об ошибке.
/// После добавления нового человека данные сохраняются в файл `persons`.
/// Пользователь может продолжать добавлять людей, пока не выберет выход.
pub fn add_people_menu() {
    let mut store = use_person_store();
    clear_console(); // Очищает консоль перед выводом меню
    loop {
        // Запрашивает имя и фамилию у пользователя
        let name = get_console("Введите имя:");
        let surname = get_console("Введите фамилию:");

        // Создает нового человека с введенными данными
        let person = match Person::new(PersonSettings { name, surname }) {
            Ok(person) => person,
            Err(error) => {
                match error {
                    EPersonError::EmptyName => println!("Поле `Имя` не должно быть пустым"),
                    EPersonError::EmptySurname => println!("Имя `Фамилия` не должно быть пустым"),
                }
                continue;
            }
        };

        // Проверяет, существует ли уже пользователь с таким идентификатором
        if store.is_exits(&person) {
            println!("Такой пользователь уже существует");
            continue; // Возвращается в начало цикла для добавления нового человека
        }

        // Выводит сообщение о добавлении пользователя
        println!(
            "Пользователь {} {} добавлен",
            person.get_name(),
            person.get_surname()
        );

        // Добавляет нового человека в Store
        store.add_person(person);

        let menu_names: [&str; 1] = ["Да"];
        // Запрашивает у пользователя, хочет ли он добавить еще одного человека
        match get_index_null("Добавить еще?", "Нет", &menu_names) {
            None => return,
            _ => clear_console(),
        }
    }
}
