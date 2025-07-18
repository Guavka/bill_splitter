use crate::store::person_store::use_user_store;

mod store;
mod utils;

const IS_DEBUG: bool = true;

fn main() {
    let store = use_user_store();
    for person in store.get_persons() {
        println!("{}", person)
    }
}
