use crate::models::bill::{Bill, BillItem, EMoneyType, HistoryBillItem};
use crate::models::person::Person;

pub fn add_persons(persons_vec: &mut Vec<Person>) {
    let mut person = Person::new("".to_string(), "".to_string());
    person.add_bill_item(HistoryBillItem {
        bill_id: "".to_string(),
        item: BillItem {
            name: "".to_string(),
            count: 1.0,
            price: 1.0,
        },
    });
    persons_vec.push(Person::new("".to_string(), "".to_string()));
}

pub fn add_bills(orders_vec: &mut Vec<Bill>) {
    let bill1 = Bill {
        id: "".to_string(),
        who_pay: 0,
        name: "".to_string(),
        date: "".to_string(),
        money_type: EMoneyType::Card,
        tips: 1.0,
        items: vec![BillItem {
            name: "".to_string(),
            count: 1.0,
            price: 1.0,
        }],
    };

    orders_vec.push(bill1);
}
