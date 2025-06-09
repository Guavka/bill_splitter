use crate::models::bill::{Bill, BillItem, EMoneyType, HistoryBillItem};
use crate::models::person::Person;

pub fn add_persons(persons_vec: &mut Vec<Person>) {
    let mut alex = Person::new("Alex".to_string(), "M".to_string());
    let alex_items = vec![
        BillItem {
            name: "Джин тоник огуречный".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Салат из телятины".to_string(),
            count: 1.0,
            price: 590.0,
        },
        BillItem {
            name: "Сет №2".to_string(),
            count: 0.5,
            price: 1980.0 / 4.0,
        },
        BillItem {
            name: "Сникерс".to_string(),
            count: 2.0,
            price: 980.0,
        },
        BillItem {
            name: "Куриные палочки".to_string(),
            count: 1.0,
            price: 410.0,
        },
        BillItem {
            name: "Соус дор блю".to_string(),
            count: 1.0,
            price: 100.0,
        },
        BillItem {
            name: "Мавританский дом".to_string(),
            count: 2.0,
            price: 1100.0,
        },
    ];

    for item in alex_items {
        alex.add_bill_item(HistoryBillItem {
            bill_id: "1".to_string(),
            item,
        })
    }

    let mut daria = Person::new("Daria".to_string(), "E".to_string());
    let daria_items = vec![
        BillItem {
            name: "Спритц Манговый".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Картофельный гратен".to_string(),
            count: 1.0,
            price: 410.0,
        },
        BillItem {
            name: "Сет №2".to_string(),
            count: 0.5,
            price: 1980.0 / 4.0,
        },
        BillItem {
            name: "Соус сырный".to_string(),
            count: 1.0,
            price: 100.0,
        },
        BillItem {
            name: "Спритц ЕПТ".to_string(),
            count: 1.0,
            price: 1100.0 / 2.0,
        },
    ];

    for item in daria_items {
        daria.add_bill_item(HistoryBillItem {
            bill_id: "1".to_string(),
            item,
        })
    }


    let mut alex_g = Person::new("Alex".to_string(), "G".to_string());
    let alex_g_items = vec![
        BillItem {
            name: "Джин тоник огуречный".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Карбонара с беконом".to_string(),
            count: 1.0,
            price: 1180.0 / 2.0,
        },
        BillItem {
            name: "Сет №2".to_string(),
            count: 2.0 / 4.0,
            price: 1980.0 / 4.0,
        },
        BillItem {
            name: "Мавританский дом".to_string(),
            count: 2.0,
            price: 1100.0,
        },
        BillItem {
            name: "Сникерс".to_string(),
            count: 1.0,
            price: 980.0 / 2.0,
        },
    ];

    for item in alex_g_items {
        alex_g.add_bill_item(HistoryBillItem {
            bill_id: "1".to_string(),
            item,
        })
    }

    let mut alexandra = Person::new("Alexandra".to_string(), "G".to_string());
    let alexandra_items = vec![
        BillItem {
            name: "Спритц Манговый".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Паста Том Ям".to_string(),
            count: 1.0,
            price: 790.0,
        },
        BillItem {
            name: "Сет №2".to_string(),
            count: 2.0 / 4.0,
            price: 1980.0 / 4.0,
        },
        BillItem {
            name: "Спритц ЕПТ".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Чизкейк".to_string(),
            count: 1.0,
            price: 410.0,
        },
    ];

    for item in alexandra_items {
        alexandra.add_bill_item(HistoryBillItem {
            bill_id: "1".to_string(),
            item,
        })
    }


    let mut arina = Person::new("Arina".to_string(), "I".to_string());
    let arina_items = vec![
        BillItem {
            name: "Авторский цитрусовый взрыв".to_string(),
            count: 1.0,
            price: 550.0,
        },
        BillItem {
            name: "Карбонара с беконом".to_string(),
            count: 1.0,
            price: 1180.0 / 2.0,
        },
    ];

    for item in arina_items {
        arina.add_bill_item(HistoryBillItem {
            bill_id: "1".to_string(),
            item,
        })
    }

    persons_vec.push(alex);
    persons_vec.push(daria);
    persons_vec.push(alex_g);
    persons_vec.push(alexandra);
    persons_vec.push(arina);
}

pub fn add_bills(orders_vec: &mut Vec<Bill>) {
    let bill1 = Bill {
        id: "1".to_string(),
        who_pay: 0,
        name: "EPT".to_string(),
        date: "31.05.2025".to_string(),
        money_type: EMoneyType::Card,
        tips: 1350.0,
        items: vec![
            BillItem {
                name: "Спритц Манговый".to_string(),
                count: 2.0,
                price: 1100.0,
            },
            BillItem {
                name: "Альмандин".to_string(),
                count: 1.0,
                price: 550.0,
            },
            BillItem {
                name: "Джин тоник огуречный".to_string(),
                count: 2.0,
                price: 1100.0,
            },
            BillItem {
                name: "Авторский цитрусовый взрыв".to_string(),
                count: 1.0,
                price: 550.0,
            },
            BillItem {
                name: "Карбонара с беконом".to_string(),
                count: 2.0,
                price: 1180.0,
            },
            BillItem {
                name: "Паста Том Ям".to_string(),
                count: 1.0,
                price: 790.0,
            },
            BillItem {
                name: "Салат из телятины".to_string(),
                count: 1.0,
                price: 590.0,
            },
            BillItem {
                name: "Картофельный гратен".to_string(),
                count: 1.0,
                price: 410.0,
            },
            BillItem {
                name: "Сет №2".to_string(),
                count: 2.0,
                price: 1980.0,
            },
            BillItem {
                name: "Соус дор блю".to_string(),
                count: 1.0,
                price: 100.0,
            },
            BillItem {
                name: "Соус сырный".to_string(),
                count: 2.0,
                price: 200.0,
            },
            BillItem {
                name: "Мавританский дом".to_string(),
                count: 4.0,
                price: 2200.0,
            },
            BillItem {
                name: "Спритц ЕПТ".to_string(),
                count: 2.0,
                price: 1100.0,
            },
            BillItem {
                name: "Чизкейк".to_string(),
                count: 1.0,
                price: 410.0,
            },
            BillItem {
                name: "Сникерс".to_string(),
                count: 2.0,
                price: 980.0,
            },
            BillItem {
                name: "Куриные палочки".to_string(),
                count: 1.0,
                price: 410.0,
            },
        ],
    };

    orders_vec.push(bill1);
}
