use crate::utils::console_io::get_number_range;
use std::fmt::Display;

pub fn show_menu<T: Display>(msg: &str, menu_items: &[T]) {
    println!("{}", msg);
    for (index, value) in menu_items.iter().enumerate() {
        println!("{}.{}", index + 1, value);
    }
}

pub fn get_index<T: Display>(msg: &str, menu_items: &[T]) -> usize {
    show_menu(msg, menu_items);

    get_number_range(
        "Выберите пункт меню:",
        1,
        menu_items.len(),
        "Ошибка: введите корректное целое число.",
        "Число должно быть в диапазоне",
    )
}

pub fn get_index_null<T: Display>(msg: &str, null_msg: &str, menu_items: &[T]) -> Option<usize> {
    show_menu(msg, menu_items);
    println!("{}.{}", menu_items.len() + 1, null_msg);

    let index = get_number_range(
        "Выберите пункт меню:",
        1,
        menu_items.len() + 1,
        "Ошибка: введите корректное целое число.",
        "Число должно быть в диапазоне",
    );
    if index == menu_items.len() + 1 {
        return None;
    }
    Some(index)
}

