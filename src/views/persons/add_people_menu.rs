use crate::store::models::person::person::{EPersonError, Person, PersonSettings};
use crate::store::persons_store::use_person_store;
use crate::utils::console_io::{clear_console, get_console};
use crate::views::view_utils::view_utils::get_index_null;

/// Функция `add_people_menu` отвечает за отображение меню для добавления новых людей в систему.
///
/// Эта функция очищает консоль, запрашивает у пользователя имя и фамилию, создает нового человека,
/// проверяет, существует ли уже пользователь с таким именем и фамилией, и добавляет его в хранилище.
/// После добавления пользователя, функция предлагает пользователю возможность добавить еще одного человека.
///
/// # Примечания
/// - Функция использует `use_person_store` для получения доступа к хранилищу пользователей.
/// - В случае ошибки при создании нового человека, выводится соответствующее сообщение об ошибке.
pub fn add_people_menu() {
    clear_console(); // Очищаем консоль перед выводом меню

    loop {
        // Запрашиваем имя и фамилию у пользователя
        let name = get_console("Введите имя:");
        let surname = get_console("Введите фамилию:");

        // Создаем нового человека с введенными данными
        let person = match Person::new(PersonSettings {
            name: name.clone(),
            surname: surname.clone(),
        }) {
            Ok(person) => person, // Успешное создание человека
            Err(error) => {
                // Обработка ошибок при создании человека
                match error {
                    EPersonError::EmptyName => println!("Поле `Имя` не должно быть пустым"),
                    EPersonError::EmptySurname => println!("Поле `Фамилия` не должно быть пустым"),
                }
                continue; // Возвращаемся в начало цикла для повторного ввода
            }
        };

        // Проверяем, существует ли уже пользователь с таким именем и фамилией
        {
            let mut store = use_person_store();
            if store.is_exits(name.clone(), surname.clone()) {
                println!("Такой пользователь уже существует");
                continue; // Возвращаемся в начало цикла для добавления нового человека
            }
            // Выводим сообщение о добавлении пользователя
            println!(
                "Пользователь {} {} добавлен",
                person.get_name(),
                person.get_surname()
            );
            store.add_person(person);
        }

        // Определяем варианты для меню
        let menu_names: [&str; 1] = ["Да"];
        // Запрашиваем у пользователя, хочет ли он добавить еще одного человека
        match get_index_null("Добавить еще?", "Нет", &menu_names) {
            None => return,       // Если пользователь выбрал "Нет", выходим из функции
            _ => clear_console(), // Если выбрано "Да", очищаем консоль для нового ввода
        }
    }
}
