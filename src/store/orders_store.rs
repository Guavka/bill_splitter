use crate::store::models::order::Order;
use std::collections::HashMap;

struct OrdersState {
    pub orders: HashMap<String, Order>,
}

pub struct OrdersStore {
    state: OrdersState,
}

impl OrdersStore {
    pub fn new() -> Self {
        Self {
            state: OrdersState {
                orders: HashMap::new(),
            },
        }
    }

    // Getter (computed property)

    // Action
}

