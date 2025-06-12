use crate::models::bill::Bill;
use crate::models::person::Person;
use crate::utils::io::get_number_range;
use crate::views::view_utils::is_exit;

pub fn show_reports_menu(persons_vec: &Vec<Person>, orders_vec: &Vec<Bill>) {
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