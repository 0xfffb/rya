use askama::Template;


#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    pub value: String
}

impl HelloTemplate {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

