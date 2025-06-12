use crate::models::bill::{BillItem, HistoryBillItem};
use crate::models::person::Person;
use crate::utils::io::get_number_positive;
use crate::utils::serialize::write_vector_to_file;
use crate::views::view_utils::is_exit;

pub fn repayment(persons_vec: &mut Vec<Person>) {
    loop {
        let index = match crate::views::view_utils::get_person_index(persons_vec) {
            Some(index) => index,
            None => continue,
        };
        let money = get_number_positive(
            "Введите сумму:",
            1.0,
            "Ошибка при чтении значения!",
            "Чаевые должны быть больше 0",
        );

        persons_vec[index].add_bill_item(HistoryBillItem {
            bill_id: "-1".to_string(),
            item: BillItem {
                name: "Возврат долга".to_string(),
                count: 1.0,
                price: -money,
            },
        });

        write_vector_to_file("persons", &*persons_vec).unwrap();
        if is_exit("Списать еще долг?\n1.Да\n2.Нет") {
            break;
        }
    }
}