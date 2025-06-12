use crate::store::models::person::short_person::ShortPerson;
use crate::store::persons_store::use_person_store;
use crate::utils::console_io::{clear_console, get_console};
use crate::views::view_utils::view_utils::get_index_null;


pub fn edit_person_menu() {
    // Определяем названия пунктов меню
    let menu_names: [&str; 2] = ["Изменить имя", "Изменить фамилию"];
    let msg = format!("Пользователь `{}`", person);
    loop {
        clear_console(); // Очищаем консоль перед отображением меню

        // Запрашиваем у пользователя выбор действия
        let index = get_index_null(msg.trim(), "Назад", &menu_names);
        match index {
            Some(index) => handle_menu_selection(index, person.clone()), // Обрабатываем выбор пользователя
            None => return, // Если пользователь выбрал "Назад", выходим из функции
        };
    }
}

/// Обрабатывает выбор пользователя в меню редактирования.
///
/// # Параметры
/// - `index`: Индекс выбранного пункта меню.
/// - `person`: Ссылка на объект `ShortPerson`, данные которого необходимо изменить.
fn handle_menu_selection(index: usize, mut person: ShortPerson) {
    let mut store = use_person_store(); // Получаем доступ к хранилищу пользователей

    match index {
        1 => {
            // Изменить имя
            if let Some(local_person) = store.get_person(person.id.clone()) {
                let name = get_console("Введите имя:"); // Запрашиваем новое имя у пользователя
                let _ = local_person.set_name(name.clone()); // Устанавливаем новое имя
                store.save_persons();

                person.name = name;
            }
        }
        2 => {
            // Изменить фамилию
            if let Some(local_person) = store.get_person(person.id.clone()) {
                let surname = get_console("Введите фамилию:"); // Запрашиваем новую фамилию у пользователя
                let _ = local_person.set_surname(surname.clone()); // Устанавливаем новую фамилию
                store.save_persons();

                person.surname = surname;
            }
        }
        _ => {} // Игнорируем любые другие значения
    }
}
