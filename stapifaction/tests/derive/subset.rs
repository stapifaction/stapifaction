use std::path::PathBuf;

use serde::Serialize;
use stapifaction::{ExpandStrategy, Persist, Persister};

use crate::common::MockPersister;

#[derive(Serialize, Persist)]
#[persist(path = "users")]
struct User {
    #[persist(id)]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    #[persist]
    #[serde(skip)]
    pub address: Address,
}

#[derive(Serialize, Persist)]
struct Address {
    pub street: String,
    pub zip_code: String,
    pub city: String,
}

#[test]
fn test_subset_in_same_folder() {
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

    persister.persist("./", &user, None).unwrap();

    persister.assert([
        PathBuf::from("./users/1/data.json"),
        PathBuf::from("./users/1/address.json"),
    ])
}

#[test]
fn test_subset_in_separate_folders() {
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

    persister
        .persist(
            "./",
            &user,
            Some(ExpandStrategy::SeparateFolders(String::from("index"))),
        )
        .unwrap();

    persister.assert([
        PathBuf::from("./users/1/index.json"),
        PathBuf::from("./users/1/address/index.json"),
    ])
}
