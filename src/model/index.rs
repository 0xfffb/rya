use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: &'static str,
    pub message: &'static str
}

impl IndexTemplate {
    
    pub fn new(title: &'static str, message: &'static str) -> Self {
        Self {
            title,
            message
        }
    }

}