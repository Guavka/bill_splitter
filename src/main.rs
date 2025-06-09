mod actions;
mod mock;
//mod mock_public;
mod models;
mod utils;

use crate::actions::menu::{
    add_order, add_people, adding_debts, get_menu_index, repayment, reports,
};
use crate::models::person::Person;
use crate::utils::io::clear_console;
use crate::utils::serialize::{read_vector_from_file, write_vector_to_file};
use models::bill::*;

const IS_DEBUG: bool = true;

fn main() {
    let mut orders_vec: Vec<Bill> = vec![];
    let mut person_vec: Vec<Person> = vec![];

    if IS_DEBUG {
        mock::add_persons(&mut person_vec);
        mock::add_bills(&mut orders_vec);

        //mock_public::add_persons(&mut person_vec);
        //mock_public::add_bills(&mut orders_vec);

        write_vector_to_file("orders", &*orders_vec).unwrap();
        write_vector_to_file("persons", &*person_vec).unwrap();
    } else {
        match read_vector_from_file("orders") {
            Ok(values) => orders_vec = values,
            _ => write_vector_to_file("orders", &*orders_vec).unwrap(),
        }
        match read_vector_from_file("persons") {
            Ok(values) => person_vec = values,
            _ => write_vector_to_file("persons", &*person_vec).unwrap(),
        }
    }

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
            3 => repayment(&mut person_vec),
            4 => adding_debts(&mut person_vec),
            5 => reports(&person_vec, &orders_vec),
            _ => return,
        }
    }
}
