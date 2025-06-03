use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EMoneyType {
    Money,
    Card
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct BillItem {
    pub name: String,
    pub count: usize,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bill {
    pub name: String,
    pub date: String,
    pub money_type: EMoneyType,
    pub items: Vec<BillItem>,
    pub tips: f64,
}
