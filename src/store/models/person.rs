use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    id: String,
    first_name: String,
    last_name: String,
    accounts: HashMap<String, f64>, // Вектор счетов представлен как словарь
}
impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        let id = Self::generate_id(first_name, last_name);
        Person {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            accounts: HashMap::new(),
        }
    }
    pub fn generate_id(first_name: &str, last_name: &str) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (first_name, last_name).hash(&mut hasher);
        hasher.finish().to_string() // Генерация id через hash-функцию
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn change_first_name(&mut self, new_first_name: String) -> Option<()> {
        if new_first_name.is_empty() {
            return None;
        }
        self.first_name = new_first_name;
        Some(())
    }

    pub fn change_last_name(&mut self, new_last_name: String) -> Option<()> {
        if new_last_name.is_empty() {
            return None;
        }
        self.last_name = new_last_name;
        Some(())
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn get_accounts(&self) -> HashMap<String, f64> {
        self.accounts.clone()
    }

    pub fn change_account(&mut self, name: String, value: f64) -> Option<()> {
        self.accounts.insert(name, value);
        Some(())
    }
}

#[derive(Debug, Clone)]
pub struct ShortPerson {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}
impl ShortPerson {
    pub fn from_person(person: &Person) -> Self {
        ShortPerson {
            id: person.get_id(),
            first_name: person.get_first_name(),
            last_name: person.get_last_name(),
        }
    }
}

impl fmt::Display for ShortPerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
