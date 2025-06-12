mod services;
mod store;
mod utils;
mod views;
use crate::views::main_menu::main_menu;

pub const IS_DEBUG: bool = true;

fn main() {
    main_menu();
}
