use crate::store::models::order::HistoryBillItem;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum EPersonError {
    EmptyName,
    EmptySurname
}

pub struct PersonSettings {
    pub name: String,
    pub surname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    id: String,
    name: String,
    surname: String,
    order_vec: Vec<HistoryBillItem>,
    credit: f32,
}

impl Person {
    pub fn new(settings: PersonSettings) -> Result<Person, EPersonError> {
        use uuid::Uuid;

        let mut person = Person {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            surname: "".to_string(),
            order_vec: vec![],
            credit: 0.0,
        };

        person.set_name(settings.name)?;
        person.set_surname(settings.surname)?;

        Ok(person)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(&mut self, name: String) -> Result<(), EPersonError> {
        if name.is_empty() {
            return Err(EPersonError::EmptyName);
        }


        self.name = name;

        Ok(())
    }

    pub fn get_surname(&self) -> String {
        self.surname.clone()
    }

    pub fn set_surname(&mut self, surname: String) -> Result<(), EPersonError> {
        if surname.is_empty() {
            return Err(EPersonError::EmptySurname);
        }

        self.surname = surname;

        Ok(())
    }

    pub fn add_bill_item(&mut self, bill_item: HistoryBillItem) {
        self.credit += bill_item.item.price;
        self.order_vec.push(bill_item);
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.surname)
    }
}
