use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate {
    pub title: String,
    pub content: String,
}
pub async fn get_index() -> IndexTemplate {
    IndexTemplate {
        title: "Hello, world!".to_string(),
        content: "Welcome to my website!".to_string(),
    }
}
