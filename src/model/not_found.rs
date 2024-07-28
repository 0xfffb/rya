use askama::Template;



#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound {
    pub message: String
}

impl NotFound {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}