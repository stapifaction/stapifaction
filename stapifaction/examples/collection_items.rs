use serde::Serialize;
use stapifaction::{json::IterableToJson, PathStyle, Persist};

#[derive(Serialize, Persist)]
struct Order {
    #[persist(id)]
    pub id: String,
    pub quantity: u64,
    #[persist]
    pub address: Address,
}

#[derive(Serialize, Persist)]
struct Address {
    pub name: String,
}

fn main() {
    let orders = vec![
        Order {
            id: String::from("OJGD"),
            quantity: 5,
            address: Address {
                name: String::from("Seatle"),
            },
        },
        Order {
            id: String::from("ZGFS"),
            quantity: 10,
            address: Address {
                name: String::from("San Fransisco"),
            },
        },
    ];

    // Each Vec items will be persisted to a different file, using the derive
    // attributes and provided path style to resolve paths to the following:
    // * `./orders/OJGD/index.json`
    // * `./orders/OJGD/address/index.json`
    // * `./orders/ZGFS/index.json`
    // * `./orders/ZGFS/address/index.json`
    orders
        .items_to_json("./orders", PathStyle::as_folders("index"))
        .unwrap();
}
