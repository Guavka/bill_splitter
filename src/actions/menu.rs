use crate::models::bill::{Bill, BillItem, EMoneyType};
use crate::models::person::Person;
use crate::utils::io::{clear_console, get_number_console, get_number_range, get_string_not_empty};

pub fn get_menu_index(menu_items: &[&str]) -> usize {
    loop {
        println!("Добро пожаловать!");
        for (index, value) in menu_items.iter().enumerate() {
            println!("{}.{}", index + 1, value);
        }

        get_number_range(
            "Выберите пункт меню:",
            1,
            menu_items.len(),
            "Ошибка: введите корректное целое число.",
            "Число должно быть в диапазоне",
        );
    }
}

pub fn is_exit(msg: &str) -> bool {
    get_number_range(
        msg,
        1,
        2,
        "Ошибка: введите корректное целое число.",
        "Число должно быть в диапазоне",
    ) == 2
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
pub fn add_people(person_vec: &mut Vec<Person>) {
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

        let person = Person::new(name.clone(), surname.clone());
        let is_not_exist = person_vec.iter().any(|p| p.get_id() == person.get_id());

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
pub fn add_order(orders_vec: &mut Vec<Bill>, persons_vec: &mut Vec<Person>) {
    /// Запрашивает у пользователя тип оплаты и возвращает его.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает значение типа `EMoneyType`, которое соответствует выбранному пользователем типу оплаты.
    fn get_money_type() -> EMoneyType {
        let money_type_index = get_number_range(
            "1.Карта\n2.Наличные",
            1,
            2,
            "Ошибка: введите корректное целое число.",
            "Число должно быть в диапазоне",
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
        println!("Кто платит?");
        for (index, person) in persons_vec.iter().enumerate() {
            println!(
                "{}.{} {}",
                index + 1,
                person.get_name(),
                person.get_surname()
            );
        }
        println!("{}. Добавить другого пользователя", persons_vec.len());

        let index = get_number_range(
            "Введите номер меню:",
            1,
            persons_vec.len(),
            "Ошибка чтения значения",
            "Число должно быть в диапазоне",
        );

        if index - 1 == persons_vec.len() {
            add_people(persons_vec);
            continue;
        }
        let who_pay: Person = persons_vec[index - 1].clone();

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
            who_pay,
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
