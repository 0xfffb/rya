use askama::Template;
use crate::model::user::User;



#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>
}

impl UsersTemplate {
    pub fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}