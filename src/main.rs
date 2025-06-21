mod services;
mod store;
mod utils;
mod views;

use crate::views::init_views;

pub const IS_DEBUG: bool = true;

fn main() {
    let menu = init_views();
    menu.show()
}
