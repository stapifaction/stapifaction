use std::path::PathBuf;

use common::MockPersister;
use stapifaction::{Persistable, Persister};

mod common;

#[derive(Persistable)]
#[persistable(path = "users")]
struct User {
    #[persistable(id)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    #[persistable(expand)]
    pub address: Address,
}

#[derive(Persistable)]
struct Address {
    pub street: String,
    pub zip_code: String,
    pub city: String,
}

#[test]
fn test_subset() {
    let persister = MockPersister::new();

    let user = User {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        address: Address {
            street: String::from("123 Main Street"),
            zip_code: String::from("ST 12345"),
            city: String::from("New York"),
        },
    };

    persister.persist("./", &user).unwrap();

    persister.assert([
        PathBuf::from("./users/1"),
        PathBuf::from("./users/1/address"),
    ])
}
