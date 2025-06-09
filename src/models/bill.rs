use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EMoneyType {
    Money,
    Card,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillItem {
    pub name: String,
    pub count: f32,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryBillItem {
    pub bill_id: String,
    pub item: BillItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bill {
    pub id: String,
    pub who_pay: usize,
    pub name: String,
    pub date: String,
    pub money_type: EMoneyType,
    pub items: Vec<BillItem>,
    pub tips: f32,
}
