use serde::Serialize;
use stapifaction::{json::ToJson, ExpandStrategy, Persist};

#[derive(Serialize, Persist)]
struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    let user = User {
        id: String::from("1"),
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };

    user.to_json("./user", ExpandStrategy::default()).unwrap()
}
