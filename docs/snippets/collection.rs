use stapifaction::Persistable;
use stapifaction::json::ToJsonIterable;

#[derive(Persistable)]
#[persistable(path = "users")]
struct User {
    #[persistable(id)]
    pub id: String;
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    let users = vec![
        User { id = "ehdz", ... },
        User { id = "ioeq", ... },
        User { id = "wqpf", ... },
    ];

    users.to_json("./").unwrap();
}