use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
pub struct About {
    pub title: String,
    pub content: String,
}

pub async fn get_about() -> About {
    About {
        title: "About".to_string(),
        content: "This is a test.".to_string(),
    }
}
