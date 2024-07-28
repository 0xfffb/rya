use askama::Template;



#[derive(Template)]
#[template(path = "404.html")]
pub struct FallbackTemplate {
    pub message: String
}

impl FallbackTemplate {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}