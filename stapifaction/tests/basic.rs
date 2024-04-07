use std::path::PathBuf;

use common::MockPersister;
use serde::Serialize;
use stapifaction::{Persist, Persister};

mod common;

#[derive(Serialize, Persist)]
struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Persist)]
struct Order {
    #[persist(id)]
    pub id: String,
    pub timestamp: u64,
}

#[test]
fn test_basic() {
    let persister = MockPersister::new();

    let user = User {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };

    persister.persist("./", &user, None).unwrap();

    persister.assert([PathBuf::from("./data")])
}

#[test]
fn test_basic_with_id() {
    let persister = MockPersister::new();

    let order = Order {
        id: String::from("1"),
        timestamp: 1703191863,
    };

    persister.persist("./", &order, None).unwrap();

    persister.assert([PathBuf::from("./1")])
}
