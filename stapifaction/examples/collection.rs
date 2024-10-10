use serde::Serialize;
use stapifaction::{json::ToJsonIterable, ExpandStrategy, Persist};

#[derive(Serialize, Persist)]
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

    orders.to_json("./orders", ExpandStrategy::default()).unwrap();
}
