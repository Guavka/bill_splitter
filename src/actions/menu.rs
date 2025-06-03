use crate::models::bill::{Bill, BillItem, EMoneyType};
use crate::models::person::Person;
use crate::utils::io::{clear_console, get_number_console, get_string_not_empty};
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
        clear_console();
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

/// Добавляет новый заказ в вектор заказов.
///
/// Эта функция запрашивает у пользователя информацию о заказе, включая название заведения,
/// дату, тип оплаты, продукты и чаевые. Все введенные данные сохраняются в векторе `orders_vec`.
///
/// # Параметры
///
/// - `orders_vec`: Изменяемая ссылка на вектор `Vec<Bill>`, в который будут добавлены новые заказы.
///
/// # Примечания
///
/// Функция будет продолжать запрашивать ввод до тех пор, пока пользователь не решит прекратить добавление заказов.
pub fn add_order(orders_vec: &mut Vec<Bill>) {
    /// Запрашивает у пользователя тип оплаты и возвращает его.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает значение типа `EMoneyType`, которое соответствует выбранному пользователем типу оплаты.
    fn get_money_type() -> EMoneyType {
        let money_type_index = get_number_console(
            "1.Карта\n2.Наличные",
            "Неверный пункт меню",
            Some(Box::new(|x: u8| -> bool {
                if x < 1 || x > 2 {
                    println!("Число должно быть от 1 до 2");
                    return false;
                }
                true
            })),
        );
        match money_type_index {
            1 => EMoneyType::Card,
            _ => EMoneyType::Money,
        }
    }
    /// Запрашивает у пользователя информацию о продуктах и возвращает вектор `Vec<BillItem>`.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает вектор `Vec<BillItem>`, содержащий информацию о продуктах в заказе.
    fn add_items() -> Vec<BillItem> {
        let mut items = vec![];
        loop {
            let item_name = get_string_not_empty(
                "Введите название продукта:",
                "Ошибка при чтении значения!",
                "Поле не должно быть пустым!",
            );
            let count =
                get_number_console("Введите количество:", "Ошибка при чтении значения!", None);
            let price = get_number_console(
                "Введите сумму:",
                "Ошибка при чтении значения!",
                Some(Box::new(|x: f64| -> bool {
                    if x <= 0.0 {
                        println!("Число должно быть больше 0");
                        return false;
                    }
                    true
                })),
            );
            items.push(BillItem {
                name: item_name,
                count,
                price,
            });
            if is_exit("Добавить еще продукт?\n1.Да\n2.Нет") {
                break;
            }
        }
        items
    }
    /// Запрашивает у пользователя сумму чаевых и возвращает ее.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает сумму чаевых типа `f64`, которая должна быть больше или равна 0.
    fn get_tips() -> f64 {
        get_number_console(
            "Введите сумму чаевых:",
            "Ошибка при чтении значения!",
            Some(Box::new(|x: f64| -> bool {
                if x < 0.0 {
                    println!("Чаевые должны быть больше 0 или равны 0");
                    return false;
                }
                true
            })),
        )
    }
    loop {
        let name = get_string_not_empty(
            "Введите название заведения:",
            "Ошибка при чтении значения!",
            "Поле не должно быть пустым!",
        );
        let date = get_string_not_empty(
            "Введите дату:",
            "Ошибка при чтении значения!",
            "Поле не должно быть пустым!",
        );
        let money_type = get_money_type();
        let items = add_items();
        let tips = get_tips();
        let bill = Bill {
            name,
            date,
            money_type,
            items,
            tips,
        };
        println!("Чек создан и добавлен: {:?}", bill);
        orders_vec.push(bill);
        if is_exit("Добавить еще чек?\n1.Да\n2.Нет") {
            break;
        }
    }
}

pub fn take_money() {}

pub fn get_money() {}
pub fn reports() {}
