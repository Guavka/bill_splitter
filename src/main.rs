mod models;

use crate::utils::io::{clear_console, get_console};
use models::bill::*;

mod utils;

fn add_people() {}
fn add_items() {}
fn take_money() {}
fn get_money() {}
fn reports() {}
fn exit() {}

fn get_menu_index(menu_items: &Vec<&str>) -> usize {
    loop {
        println!("Добро пожаловать!");
        for (index, value) in menu_items.iter().enumerate() {
            println!("{}.{}", index + 1, value);
        }

        let menu_len = menu_items.len();
        let index: usize = get_console(
            "Введите целое число:",
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

fn main() {
    let bill = vec![Bill {
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

    let menu_names = vec![
        "Добавление участников",
        "Добавление позиций",
        "Погашение долгов",
        "Добавление долгов",
        "Отчеты",
        "Выход",
    ];

    let menu_actions = vec![add_people, add_items, take_money, get_money, reports, exit];

    loop {
        clear_console();

        let index = get_menu_index(&menu_names);

        if let Some(func) = menu_actions.get(index - 1) {
            func();
        }
    }
}
