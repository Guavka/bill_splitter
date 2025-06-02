mod models;

use crate::models::person::Person;
use crate::utils::io::{clear_console, get_number_console, get_string_console};
use models::bill::*;
use std::collections::HashSet;

mod utils;

fn get_menu_index(menu_items: &Vec<&str>) -> usize {
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

fn is_exit(msg: &str) -> bool {
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

    let add_people: Box<dyn FnMut()> = Box::new(|| loop {
        let get_string = |msg: &str| -> String {
            get_string_console(
                msg,
                "Возникла проблема при чтении значения",
                Some(Box::new(|x: &String| -> bool {
                    if x.is_empty() {
                        println!("{}", "Поле не должно быть пустым");
                        return false;
                    }
                    true
                })),
            )
        };

        clear_console();
        let name: String = get_string("Введите имя:");
        let surname: String = get_string("Введите фамилию:");

        let is_not_exist = person_set.insert(Person::new(name.clone(), surname.clone()));
        if is_not_exist {
            println!("Пользователь {} {} добавлен", name, surname);
        } else {
            println!("Такой пользователь уже существует");
        }

        if is_exit("Добавить еще?\n1.Да\n2.Нет") {
            break;
        }
    });

    let take_money: Box<dyn FnMut()> = Box::new(|| {});
    let add_items: Box<dyn FnMut()> = Box::new(|| {});
    let get_money: Box<dyn FnMut()> = Box::new(|| {});
    let reports: Box<dyn FnMut()> = Box::new(|| {});
    let exit: Box<dyn FnMut()> = Box::new(|| {});

    let menu_names = vec![
        "Добавление участников",
        "Добавление позиций",
        "Погашение долгов",
        "Добавление долгов",
        "Отчеты",
        "Выход",
    ];

    let mut menu_actions = vec![add_people, add_items, take_money, get_money, reports, exit];

    loop {
        clear_console();

        let index = get_menu_index(&menu_names);
        menu_actions[index - 1]();
    }
}
