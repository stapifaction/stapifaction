use stapifaction::{json::ToJson, Persist};

#[derive(Persist)]
struct User {
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    let user = User::new();

    user.to_json("./").unwrap();
}
