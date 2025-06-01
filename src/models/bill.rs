use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EMoneyType {
    Money,
    Card
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct BillItem {
    pub name: String,
    pub count: u32,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bill {
    pub name: String,
    pub date: String,
    pub money_type: EMoneyType,
    pub items: Vec<BillItem>,
    pub tips: f64,
    pub is_auto_tips: bool
}
