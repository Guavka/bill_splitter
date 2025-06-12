use crate::utils::console_io::clear_console;
use crate::views::persons::edit_person_menu::edit_person_menu;
use crate::views::persons::remove_person_menu::remove_person_menu;
use crate::views::view_utils::view_utils::get_index_null;


pub fn select_person_menu() {
    // Определяем названия пунктов меню
    let menu_names: [&str; 2] = ["Редактировать", "Удалить"];
    // Формируем сообщение с именем пользователя
    let msg = format!("Пользователь `{}`", &person);

    loop {
        clear_console(); // Очищаем консоль перед отображением меню

        // Запрашиваем у пользователя выбор действия
        let index = get_index_null(msg.trim(), "Назад", &menu_names);
        match index {
            Some(index) => match index {
                // Если выбран пункт "Редактировать", вызываем соответствующую функцию
                1 => edit_person_menu(person.clone()),
                // Если выбран пункт "Удалить", вызываем функцию для удаления пользователя и выходим
                2 => {
                    remove_person_menu(person.clone());
                    return;
                }
                _ => {} // Игнорируем любые другие значения
            },
            // Если пользователь выбрал "Назад", выходим из функции
            None => return,
        }
    }
}

