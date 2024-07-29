use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub message: String
}

impl IndexTemplate {
    
    pub fn new(title: String, message: String) -> Self {
        Self { title, message }
    }
}