use crate::models::bill::HistoryBillItem;

#[derive(Debug)]
pub struct Person {
    id: String,
    name: String,
    surname: String,
    order_vec: Vec<HistoryBillItem>,
    credit: f32,
}

impl Person {
    pub fn new(name: String, surname: String) -> Person {
        use uuid::Uuid;
        
        Person {
            id: Uuid::new_v4().to_string(),
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

    pub fn add_bill_item(&mut self, bill_item: HistoryBillItem) {
        self.credit += bill_item.item.price;
        self.order_vec.push(bill_item);
    }
}
