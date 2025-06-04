mod actions;
mod models;
mod utils;

use crate::actions::menu::{add_order, add_people, get_menu_index, get_money, reports, take_money};
use crate::models::person::Person;
use crate::utils::io::clear_console;
use models::bill::*;

fn main() {
    let mut orders_vec: Vec<Bill> = vec![];
    let mut person_vec: Vec<Person> = vec![];

    let menu_names: [&str; 6] = [
        "Добавление участников",
        "Добавление чека",
        "Погашение долгов",
        "Добавление долгов",
        "Отчеты",
        "Выход",
    ];

    loop {
        clear_console();

        let index = get_menu_index(&menu_names);

        match index {
            1 => add_people(&mut person_vec),
            2 => add_order(&mut orders_vec, &mut person_vec),
            3 => take_money(),
            4 => get_money(),
            5 => reports(),
            _ => return,
        }
    }
}
