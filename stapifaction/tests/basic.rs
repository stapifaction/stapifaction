use std::path::PathBuf;

use common::MockPersister;
use stapifaction::{Persistable, Persister};

mod common;

#[derive(Persistable)]
struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
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
