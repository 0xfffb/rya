

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

impl User {
    pub fn new(id: i32, username: String, password: String) -> Self {
        Self { id, username, password }
    }
}