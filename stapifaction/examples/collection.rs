use serde::Serialize;
use stapifaction::{json::ToJson, Persist};

#[derive(Serialize, Persist)]
struct Order {
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
            id: String::from("ZGFS"),
            quantity: 5,
            address: Address {
                name: String::from("Seatle"),
            },
        },
        Order {
            id: String::from("OJGD"),
            quantity: 10,
            address: Address {
                name: String::from("San Fransisco"),
            },
        },
    ];

    // The orders Vec will be persisted as a single file, using the path style
    // to specify the file name.
    orders.to_json_with_path_style("./", "orders").unwrap();
}
