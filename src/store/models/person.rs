use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum EPersonErrors {
    EmptyName,
    InvalidName,
    EmptySurname,
    InvalidSurname,
    InvalidAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    id: String,
    first_name: String,
    last_name: String,
    accounts: HashMap<String, f64>, // Вектор счетов представлен как словарь
}
impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Result<Self, EPersonErrors> {
        let id = Self::generate_id(first_name, last_name);
        let mut person = Person {
            id,
            first_name: String::new(),
            last_name: String::new(),
            accounts: HashMap::new(),
        };
        person.change_first_name(first_name.to_string())?;
        person.change_last_name(last_name.to_string())?;

        Ok(person)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    fn check_name(name: String) -> Result<String, EPersonErrors> {
        if name.is_empty() {
            return Err(EPersonErrors::EmptyName);
        }
        if !name.chars().next().unwrap().is_alphabetic() {
            return Err(EPersonErrors::InvalidName);
        }
        Ok(name)
    }

    pub fn change_first_name(&mut self, new_first_name: String) -> Result<String, EPersonErrors> {
        self.first_name = Self::check_name(new_first_name.clone())?;
        Ok(new_first_name)
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }
    pub fn change_last_name(&mut self, new_last_name: String) -> Result<String, EPersonErrors> {
        self.last_name = Person::check_name(new_last_name.clone())?;
        Ok(new_last_name)
    }

    pub fn get_accounts(&self) -> HashMap<String, f64> {
        self.accounts.clone()
    }

    pub fn increase_account(&mut self, id: String, amount: f64) -> Result<f64, EPersonErrors> {
        if amount < 1f64 {
            return Err(EPersonErrors::InvalidAmount);
        }
        match self.accounts.get(&id) {
            None => self.accounts.insert(id, amount),
            Some(value) => self.accounts.insert(id, value + amount),
        };
        Ok(amount)
    }

    pub fn decrease_account(&mut self, id: String, amount: f64) -> Result<f64, EPersonErrors> {
        if amount < 1f64 {
            return Err(EPersonErrors::InvalidAmount);
        }
        match self.accounts.get(&id) {
            None => self.accounts.insert(id, amount),
            Some(value) => self.accounts.insert(id, value - amount),
        };
        Ok(amount)
    }

    fn generate_id(first_name: &str, last_name: &str) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (first_name, last_name).hash(&mut hasher);
        hasher.finish().to_string() // Генерация id через hash-функцию
    }
}

// Реализация PartialEq для сравнения только по имени и фамилии
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name && self.last_name == other.last_name
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
