use std::path::PathBuf;

use stapifaction::{Persistable, Persister};

use common::MockPersister;

mod common;

#[derive(Persistable)]
#[persistable(path = "products")]
struct Product {
    #[persistable(id)]
    pub id: u64,
    pub label: String,
    pub price: u64,
    #[persistable(expand = "all")]
    pub factories: Vec<Factory>,
    #[persistable(expand = "all")]
    pub orders: Vec<Order>,
}

#[derive(Persistable)]
struct Factory {
    #[persistable(id)]
    pub id: u64,
    pub name: String,
}

#[derive(Persistable)]
#[persistable(expand_strategy = "id-only")]
struct Order {
    #[persistable(id)]
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
        PathBuf::from("./products/1/data"),
        PathBuf::from("./products/1/factories/10/data"),
        PathBuf::from("./products/1/factories/20/data"),
        PathBuf::from("./products/1/factories/30/data"),
        PathBuf::from("./products/1/orders/ZGFS"),
        PathBuf::from("./products/1/orders/OJGD"),
    ])
}
