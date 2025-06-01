use std::collections::HashMap;
use crate::models::bill::BillItem;

#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub order: Vec<BillItem>,
    pub relationship: HashMap<String, String>
}