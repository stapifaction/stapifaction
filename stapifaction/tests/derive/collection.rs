use std::path::PathBuf;

use serde::Serialize;
use stapifaction::{Persist, Persister};

use crate::common::MockPersister;

#[derive(Serialize, Persist)]
#[persist(path = "products")]
struct Product {
    #[persist(id)]
    pub id: u64,
    pub label: String,
    pub price: u64,
    #[persist(expand = "all")]
    #[serde(skip)]
    pub factories: Vec<Factory>,
    #[persist(expand = "all")]
    #[serde(skip)]
    pub orders: Vec<Order>,
}

#[derive(Serialize, Persist)]
struct Factory {
    #[persist(id)]
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Persist)]
struct Order {
    #[persist(id)]
    pub id: String,
    pub quantity: u64,
}

#[test]
fn test_collection() {
    let persister = MockPersister::new();

    let product = Product {
        id: 1,
        label: String::from("Phone"),
        price: 600,
        factories: vec![
            Factory {
                id: 10,
                name: String::from("London"),
            },
            Factory {
                id: 20,
                name: String::from("Tokyo"),
            },
            Factory {
                id: 30,
                name: String::from("Berlin"),
            },
        ],
        orders: vec![
            Order {
                id: String::from("ZGFS"),
                quantity: 5,
            },
            Order {
                id: String::from("OJGD"),
                quantity: 10,
            },
        ],
    };

    persister.persist("./", &product, None).unwrap();

    persister.assert([
        PathBuf::from("./products/1/data.json"),
        PathBuf::from("./products/1/factories/10.json"),
        PathBuf::from("./products/1/factories/20.json"),
        PathBuf::from("./products/1/factories/30.json"),
        PathBuf::from("./products/1/orders/ZGFS.json"),
        PathBuf::from("./products/1/orders/OJGD.json"),
    ])
}
