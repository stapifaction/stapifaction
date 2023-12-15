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
}

#[derive(Persistable)]
struct Factory {
    #[persistable(id)]
    pub id: u64,
    pub name: String,
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
    };

    persister.persist("./", &product).unwrap();

    persister.assert([
        PathBuf::from("./products/1"),
        PathBuf::from("./products/1/factories/10"),
        PathBuf::from("./products/1/factories/20"),
        PathBuf::from("./products/1/factories/30"),
    ])
}
