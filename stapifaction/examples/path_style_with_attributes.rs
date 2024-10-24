use serde::Serialize;
use stapifaction::{json::ToJson, Persist};

#[derive(Serialize, Persist)]
#[persist(as_folders)]
#[persist(file_name = "i")]
struct Factory {
    pub name: String,
    #[serde(skip)]
    #[persist]
    pub location: Address,
}

#[derive(Serialize, Persist)]
struct Address {
    pub street: String,
    pub zip_code: String,
    pub city: String,
    pub country: String,
}

fn main() {
    let factory = Factory {
        name: String::from("Pen factory"),
        location: Address {
            street: String::from("123 Main Street"),
            zip_code: String::from("ST 12345"),
            city: String::from("New York"),
            country: String::from("USA"),
        },
    };

    // This will use the derive attributes on `Factory` to persist this struct
    // as 2 files:
    // * `./factory/i.json`
    // * `./factory/location/i.json`
    factory.to_json("./factory").unwrap()
}
