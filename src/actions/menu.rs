use crate::models::bill::{Bill, BillItem, EMoneyType, HistoryBillItem};
use crate::models::person::Person;
use crate::utils::io::{
    clear_console, get_number_console, get_number_positive, get_number_range, get_string_not_empty,
};

pub fn get_menu_index(menu_items: &[&str]) -> usize {
    loop {
        println!("Добро пожаловать!");
        for (index, value) in menu_items.iter().enumerate() {
            println!("{}.{}", index + 1, value);
        }

        return get_number_range(
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
/// - `person_vec`: Изменяемая ссылка на множество `&mut Vec<Person>`,
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
        let is_exist = person_vec.iter().any(|p| p.get_id() == person.get_id());

        if is_exist {
            println!("Такой пользователь уже существует");
        } else {
            println!("Пользователь {} {} добавлен", name, surname);
            person_vec.push(person);
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

    /// Добавляет товары в счет и распределяет их между пользователями.
    ///
    /// Эта функция запрашивает у пользователя ввод информации о товарах, включая название, количество и цену.
    /// Затем она распределяет товары между пользователями на основе их выбора.
    /// Функция продолжает запрашивать ввод до тех пор, пока пользователь не решит завершить добавление товаров.
    ///
    /// # Параметры
    ///
    /// - `main_pay_index`: Индекс пользователя, который будет основным плательщиком за товары.
    /// - `persons_vec`: Изменяемая ссылка на вектор `Vec<Person>`, содержащий список пользователей.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает вектор `Vec<BillItem>`, содержащий добавленные товары.
    ///
    /// # Примечания
    ///
    /// Функция будет продолжать запрашивать ввод до тех пор, пока не будет получена корректная информация о товарах.
    /// Если пользователь решает разделить товар между несколькими людьми, функция будет запрашивать, сколько
    /// каждый человек должен получить. Если товар полностью распределен, он добавляется в счет основного плательщика.
    fn add_items(
        main_pay_index: usize,
        bill_id: String,
        persons_vec: &mut Vec<Person>,
    ) -> Vec<BillItem> {
        let mut items = vec![];
        loop {
            let name = get_string_not_empty(
                "Введите название продукта:",
                "Ошибка при чтении значения!",
                "Поле не должно быть пустым!",
            );
            let count =
                get_number_console("Введите количество:", "Ошибка при чтении значения!", None);
            let price = get_number_positive(
                "Введите сумму:",
                0.0,
                "Ошибка при чтении значения!",
                "Число должно быть больше",
            );

            let mut local_count = count;
            loop {
                let who_pay = match get_person_index(persons_vec) {
                    Some(index) => index,
                    None => continue,
                };

                let person_count = get_number_range(
                    "Введите количество",
                    1.0,
                    local_count,
                    "Ошибка при чтении значения!",
                    "Число должно быть в диапазоне",
                );

                let item = BillItem {
                    name: name.clone(),
                    count: person_count,
                    price: price * person_count * (1.0 / count),
                };

                persons_vec[who_pay].add_bill_item(HistoryBillItem {
                    bill_id: bill_id.clone(),
                    item,
                });
                local_count -= person_count;

                if local_count == 0.0 {
                    break;
                }

                if is_exit("Разделить еще?\n1.Да\n2.Нет") {
                    let item = BillItem {
                        name: name.clone(),
                        count: local_count,
                        price: price * local_count * (1.0 / count),
                    };
                    persons_vec[main_pay_index].add_bill_item(HistoryBillItem {
                        bill_id: bill_id.clone(),
                        item,
                    });
                    break;
                }
            }

            items.push(BillItem { name, count, price });
            if is_exit("Добавить еще продукт?\n1.Да\n2.Нет") {
                break;
            }
        }
        items
    }

    /// Запрашивает у пользователя выбор человека из списка и возвращает индекс выбранного человека.
    ///
    /// Эта функция выводит список людей из вектора `persons_vec` и запрашивает у пользователя
    /// ввод номера выбранного человека. Если пользователь выбирает опцию для добавления нового
    /// человека, вызывается функция `add_people`, и функция возвращает `None`.
    /// В противном случае возвращается индекс выбранного человека.
    ///
    /// # Параметры
    ///
    /// - `persons_vec`: Изменяемая ссылка на вектор `Vec<Person>`, содержащий список людей.
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает `Option<usize>`, где:
    /// - `Some(usize)` — индекс выбранного человека в векторе
    /// - `None` — если пользователь выбрал опцию для добавления нового человека.
    ///
    /// # Примечания
    ///
    /// Функция будет продолжать запрашивать ввод до тех пор, пока не будет получен корректный номер
    /// из меню. Если введенный номер не соответствует ни одному из существующих людей,
    /// функция будет ожидать повторного ввода.
    fn get_person_index(persons_vec: &mut Vec<Person>) -> Option<usize> {
        println!("Кто платит?");
        for (index, person) in persons_vec.iter().enumerate() {
            println!(
                "{}.{} {}",
                index + 1,
                person.get_name(),
                person.get_surname()
            );
        }
        println!("{}.Добавить пользователя", persons_vec.len() + 1);

        let index = get_number_range(
            "Введите номер меню:",
            1,
            persons_vec.len() + 1,
            "Ошибка чтения значения",
            "Число должно быть в диапазоне",
        );

        if index - 1 == persons_vec.len() {
            add_people(persons_vec);
            return None;
        }

        Some(index - 1)
    }

    loop {
        use uuid::Uuid;

        let id = Uuid::new_v4().to_string();

        let who_pay = match get_person_index(persons_vec) {
            Some(index) => index,
            None => continue,
        };

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
        let items = add_items(who_pay, id.clone(), persons_vec);

        let tips = get_number_positive(
            "Введите сумму чаевых:",
            1.0,
            "Ошибка при чтении значения!",
            "Чаевые должны быть больше 0",
        );

        let bill = Bill {
            id,
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
pub fn reports(persons_vec: &Vec<Person>, orders_vec: &Vec<Bill>) {
    fn report_people(persons_vec: &Vec<Person>) {
        loop {
            for (index, person) in persons_vec.iter().enumerate() {
                println!("{}.{:?}", index, person);
            }
            if !is_exit("Назад?\n1.Да\n2.Нет") {
                break;
            }
        }
    }

    fn report_bills(orders_vec: &Vec<Bill>) {
        loop {
            for (index, bill) in orders_vec.iter().enumerate() {
                println!("{}.{:?}", index, bill);
            }
            if !is_exit("Назад?\n1.Да\n2.Нет") {
                break;
            }
        }
    }

    loop {
        let choose = get_number_range(
            "1.Отчет по пользователям\n2.Отчет по чекам\n3.Назад",
            1,
            3,
            "Ошибка: введите корректное целое число.",
            "Число должно быть в диапазоне",
        );

        match choose {
            1 => report_people(&persons_vec),
            2 => report_bills(&orders_vec),
            _ => break,
        }
    }
}
