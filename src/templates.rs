use askama::Template;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Template, Clone)]
#[template(path = "course.html", escape = "none")]
pub struct Course {
    pub lang: String,
    pub title: String,
    pub url: String,
    pub tutorials: Vec<Tutorial>,
}

fn apply_markdown<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use pulldown_cmark::{html, Parser};

    let content = String::deserialize(deserializer)?;
    let parser = Parser::new(&content);

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    Ok(html_buf)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    pub subtitle: String,
    #[serde(deserialize_with = "apply_markdown")]
    pub content: String,
}
