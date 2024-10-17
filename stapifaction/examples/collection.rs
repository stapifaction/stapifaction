use serde::Serialize;
use stapifaction::{json::ToJsonIterable, Persist};

#[derive(Serialize, Persist)]
#[persist(as_folders)]
struct Order {
    #[persist(id)]
    pub id: String,
    pub quantity: u64,
}

fn main() {
    let orders = vec![
        Order {
            id: String::from("ZGFS"),
            quantity: 5,
        },
        Order {
            id: String::from("OJGD"),
            quantity: 10,
        },
    ];

    orders.items_to_json("./orders").unwrap();
}
