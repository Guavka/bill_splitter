use crate::store::mock::persons_mock::get_mock_persons;
use crate::store::models::person::{Person, ShortPerson};
use crate::utils::serialize::{read_hashmap_from_file, write_hashmap_to_file};
use crate::IS_DEBUG;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

pub struct PersonsStore {
    state: HashMap<String, Person>, // Хранилище пользователей по id
}
impl PersonsStore {
    // Getter (computed property)
    pub fn get_persons(&self) -> Vec<ShortPerson> {
        let mut persons_vec = vec![];
        for person in self.state.values() {
            persons_vec.push(ShortPerson::from_person(person));
        }
        persons_vec
    }
    pub fn is_exits(&self, person: &Person) -> bool {
        self.state.values().any(|value| {
            value.get_first_name() == person.get_first_name()
                && value.get_last_name() == person.get_last_name()
        })
    }
    // Action
    pub fn add_person(&mut self, person: Person) {
        self.state.insert(person.get_id(), person);
    }
    fn save_persons(&self) {
        write_hashmap_to_file("persons", &self.state).unwrap();
    }
    fn load_persons(&mut self) {
        if IS_DEBUG {
            self.state = get_mock_persons();
            self.save_persons();
        } else {
            match read_hashmap_from_file("persons") {
                Ok(values) => self.merge_map(values),
                _ => self.save_persons(),
            }
        }
    }

    fn merge_map(&mut self, values: HashMap<String, Person>) {
        for (key, value) in values {
            self.state.insert(key, value);
        }
    }
}

// Статическая переменная с мьютексом
static SINGLETON: Lazy<Mutex<PersonsStore>> = Lazy::new(|| {
    let mut persons = PersonsStore {
        state: HashMap::new(),
    };
    persons.load_persons();
    Mutex::new(persons)
});

pub fn use_user_store() -> MutexGuard<'static, PersonsStore> {
    SINGLETON.try_lock().unwrap()
}
