use serde::Serialize;
use stapifaction::{json::ToJson, PathStyle, Persist};

#[derive(Serialize, Persist)]
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

    factory
        .to_json_with_path_style("./factory", PathStyle::as_folders("i"))
        .unwrap()
}
