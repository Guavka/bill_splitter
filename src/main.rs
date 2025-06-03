mod actions;
mod models;
mod utils;

use crate::actions::menu::{add_items, add_people, get_menu_index, get_money, reports, take_money};
use crate::models::person::Person;
use crate::utils::io::clear_console;
use models::bill::*;
use std::collections::HashSet;

fn main() {
    let mock_bill = vec![Bill {
        name: "ЕПТ".to_string(),
        date: "25.05.2025".to_string(),
        items: vec![
            BillItem {
                name: "Бланш ЕПТ".to_string(),
                count: 2,
                price: 2000.0,
            },
            BillItem {
                name: "Космополитен".to_string(),
                count: 3,
                price: 1470.0,
            },
            BillItem {
                name: "Картофель фри".to_string(),
                count: 1,
                price: 270.0,
            },
            BillItem {
                name: "Соус чесночный".to_string(),
                count: 1,
                price: 100.0,
            },
            BillItem {
                name: "Собачкина столица".to_string(),
                count: 3,
                price: 1650.0,
            },
            BillItem {
                name: "Спритц манговый".to_string(),
                count: 1,
                price: 550.0,
            },
            BillItem {
                name: "Аврора".to_string(),
                count: 1,
                price: 550.0,
            },
        ],
        tips: 0.0,
        money_type: EMoneyType::Card,
        is_auto_tips: true,
    }];
    let mut person_set: HashSet<Person> = HashSet::new();

    let menu_names: [&str; 6] = [
        "Добавление участников",
        "Добавление позиций",
        "Погашение долгов",
        "Добавление долгов",
        "Отчеты",
        "Выход",
    ];

    loop {
        clear_console();

        let index = get_menu_index(&menu_names) - 1;

        match index {
            1 => add_people(&mut person_set),
            2 => add_items(),
            3 => take_money(),
            4 => get_money(),
            5 => reports(),
            _ => return,
        }
    }
}
