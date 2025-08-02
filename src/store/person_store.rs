use crate::store::mock::persons_mock::get_mock_persons;
use crate::store::models::person::{Person, ShortPerson};
use crate::utils::serialize::{read_hashmap_from_file, write_hashmap_to_file};
use crate::IS_DEBUG;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

const FILE_NAME: &str = "persons";

pub enum EPersonsStoreErrors {
    IsNotExist,
    AlreadyExist,
}

pub struct PersonsStore {
    state: HashMap<String, Person>, // Хранилище пользователей по id
}
impl PersonsStore {
    // Getter (computed property)

    pub fn is_exist(&self, other: &Person) -> bool {
        self.state.values().any(|person| {
            person.get_id() == other.get_id()
                && person.get_last_name() == other.get_last_name()
                && person.get_first_name() == other.get_first_name()
        })
    }
    pub fn get_person(&self, id: String) -> Result<&Person, EPersonsStoreErrors> {
        match self.state.get(&id) {
            None => Err(EPersonsStoreErrors::IsNotExist),
            Some(person) => Ok(person),
        }
    }

    pub fn get_short_info(&self) -> Vec<ShortPerson> {
        let mut persons_vec = vec![];
        for person in self.state.values() {
            persons_vec.push(ShortPerson::from_person(person));
        }
        persons_vec
    }
    // Action
    pub fn add_person(&mut self, person: Person) -> Result<(), EPersonsStoreErrors> {
        match self.state.get(&person.get_id()) {
            None => {
                self.state.insert(person.get_id(), person);
                self.save_persons();
                Ok(())
            }
            Some(_) => Err(EPersonsStoreErrors::AlreadyExist),
        }
    }

    pub fn remove_person(&mut self, id: String) -> Result<(), EPersonsStoreErrors> {
        match self.state.get(&id) {
            None => Err(EPersonsStoreErrors::IsNotExist),
            Some(_) => {
                self.state.remove(&id);
                self.save_persons();
                Ok(())
            }
        }
    }

    pub fn save_persons(&self) {
        write_hashmap_to_file(FILE_NAME, &self.state).unwrap();
    }

    fn load_persons(&mut self) {
        if IS_DEBUG {
            self.state = get_mock_persons();
            self.save_persons();
        } else {
            match read_hashmap_from_file(FILE_NAME) {
                Ok(values) => {
                    for (key, value) in values {
                        self.state.insert(key, value);
                    }
                }
                _ => self.save_persons(),
            }
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
