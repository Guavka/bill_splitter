use crate::store::models::mock::mock_persons::get_mock_persons;
use crate::store::models::person::{Person, ShortPerson};
use crate::utils::serialize::{merge_map, read_hashmap_from_file, write_hashmap_to_file};
use crate::IS_DEBUG;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

// Определяем статическую переменную с ленивой инициализацией
static SINGLETON: Lazy<Mutex<PersonsStore>> = Lazy::new(|| {
    let mut persons = PersonsStore {
        state: HashMap::new(),
    };
    persons.load_persons();
    Mutex::new(persons)
});

pub struct PersonsStore {
    state: HashMap<String, Person>,
}

impl PersonsStore {
    // Getter (computed property)
    pub fn get_persons(&self) -> Vec<ShortPerson> {
        self.state
            .values()
            .map(|person| ShortPerson {
                id: person.get_id(),
                name: person.get_name(),
                surname: person.get_surname(),
            })
            .collect()
    }

    pub fn is_exits(&self, person: &Person) -> bool {
        for value in self.state.values() {
            if value.get_surname() == person.get_surname() && value.get_name() == person.get_name()
            {
                return true;
            }
        }

        false
    }

    // Action

    pub fn get_person(&self, id: String) -> Option<&Person> {
        self.state.get(&id)
    }

    pub fn remove_person(&mut self, id: String) {
        let _ = &self.state.remove(&id);
        self.save_persons();
    }

    pub fn add_person(&mut self, person: Person) {
        let _ = &self.state.insert(person.get_id(), person);
        self.save_persons();
    }

    fn save_persons(&self) {
        write_hashmap_to_file("persons", &self.state).unwrap();
    }

    fn load_persons(&mut self) {
        if IS_DEBUG {
            self.state = get_mock_persons().unwrap();
            self.save_persons();
        } else {
            match read_hashmap_from_file("persons") {
                Ok(values) => merge_map(&mut self.state, values),
                _ => self.save_persons(),
            }
        }
    }
}

pub fn use_person_store() -> MutexGuard<'static, PersonsStore> {
    SINGLETON.lock().unwrap()
}
