#[derive(Persist)]
struct User {
    #[persist(id)] // [!code ++]
    pub id: String;
    pub first_name: String,
    pub last_name: String,
}
