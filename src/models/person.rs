use crate::models::bill::BillItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    id: String,
    name: String,
    surname: String,
    order_vec: Vec<BillItem>,
    credit: f64,
}

impl Person {
    pub fn new(name: String, surname: String) -> Person {
        let id = name.clone() + surname.trim();
        Person {
            id,
            name,
            surname,
            order_vec: vec![],
            credit: 0.0,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_surname(&self) -> String {
        self.surname.clone()
    }

    pub fn add_bill_item(&mut self, bill_item: BillItem) {
        self.credit += bill_item.price;
        self.order_vec.push(bill_item);
    }
}
