use crate::store::models::person::short_person::ShortPerson;
use crate::store::persons_store::use_person_store;
use crate::utils::console_io::{clear_console, get_console};
use crate::views::view_utils::view_utils::get_index_null;

pub fn edit_person_menu(person: &ShortPerson) {
    // Определяем названия пунктов меню
    let menu_names: [&str; 2] = ["Изменить имя", "Изменить фамилию"];

    let mut person = person.clone();

    loop {
        clear_console(); // Очищаем консоль перед отображением меню

        let msg = format!("Пользователь `{}`", person);

        // Запрашиваем у пользователя выбор действия
        let index = get_index_null(msg.trim(), "Назад", &menu_names);
        match index {
            Some(index) => {
                match index {
                    1 => person.name = change_name(&person),
                    2 => person.surname = change_surname(&person),
                    _ => {} // Игнорируем любые другие значения
                };
            }
            None => return, // Если пользователь выбрал "Назад", выходим из функции
        };
    }
}

/// Изменяет имя пользователя.
///
/// # Параметры
/// - `person`: Ссылка на объект `ShortPerson`, данные которого необходимо изменить.
fn change_name(person: &ShortPerson) -> String {
    let mut store = use_person_store();
    loop {
        let name = get_console("Введите имя:"); // Запрашиваем новое имя у пользователя

        // Проверяем, существует ли уже пользователь с таким именем и фамилией
        if store.is_exits(name.clone(), person.surname.clone()) {
            println!("Такой пользователь уже существует");
            continue;
        }

        // Устанавливаем новое имя
        if let Some(local_person) = store.get_person(&person.id) {
            local_person
                .set_name(name.clone())
                .expect("Не удалось установить имя")
        }
        store.save_persons();
        println!("Имя изменено");

        return name;
    }
}

/// Изменяет фамилию пользователя.
///
/// # Параметры
/// - `person`: Ссылка на объект `ShortPerson`, данные которого необходимо изменить.
fn change_surname(person: &ShortPerson) -> String {
    let mut store = use_person_store();
    loop {
        let surname = get_console("Введите фамилию:"); // Запрашиваем новое имя у пользователя

        // Проверяем, существует ли уже пользователь с таким именем и фамилией
        if store.is_exits(person.name.clone(), surname.clone()) {
            println!("Такой пользователь уже существует");
            continue;
        }

        // Устанавливаем новую фамилию
        if let Some(local_person) = store.get_person(&person.id) {
            local_person
                .set_surname(surname.clone())
                .expect("Не удалось установить фамилию");
        }
        store.save_persons();
        println!("Фамилия изменена");

        return surname;
    }
}
