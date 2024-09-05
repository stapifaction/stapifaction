use stapifaction::Persistable;

#[derive(Persistable)]
struct User {
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    let user = User::new();

    user.to_json("./").unwrap();
}
