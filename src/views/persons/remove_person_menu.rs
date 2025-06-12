use crate::store::models::person::ShortPerson;
use crate::store::persons_store::use_person_store;
use crate::utils::console_io::get_console;
use crate::views::view_utils::view_utils::get_index_null;

pub fn remove_person_menu(person: &ShortPerson) {
    let menu_names: [&str; 1] = ["Да"];

    let msg = format!(
        "Вы точно хотите удалить пользователя {}?\nДанное решение отменить нельзя.",
        person
    );

    let index = get_index_null(msg.trim(), "Назад", &menu_names);

    match index {
        None => return,
        _ => {
            let mut store = use_person_store();
            store.remove_person(person.id.clone());
            println!("Пользователь был удален");
            get_console("Нажмите любую клавишу...");
            return;
        }
    }
}
