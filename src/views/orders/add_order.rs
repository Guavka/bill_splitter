use crate::models::bill::{Bill, BillItem, EMoneyType, HistoryBillItem};
use crate::models::person::Person;
use crate::utils::io::{get_number_console, get_number_positive, get_number_range, get_string_not_empty};
use crate::utils::serialize::write_vector_to_file;
use crate::views::view_utils::is_exit;

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
                let who_pay = match crate::views::view_utils::get_person_index(persons_vec) {
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

    loop {
        use uuid::Uuid;

        let id = Uuid::new_v4().to_string();

        let who_pay = match crate::views::view_utils::get_person_index(persons_vec) {
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
            0.0,
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

        write_vector_to_file("orders", &*orders_vec).unwrap();
        write_vector_to_file("persons", &*persons_vec).unwrap();

        if is_exit("Добавить еще чек?\n1.Да\n2.Нет") {
            break;
        }
    }
}