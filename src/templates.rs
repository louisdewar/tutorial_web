use askama::Template;
use std::collections::HashMap;

#[derive(Template, Clone)]
#[template(path = "course.html", escape = "none")]
/// Struct used for rendering a course
pub struct Page {
    pub base_url: String,
    pub course: Course,
}

#[derive(Debug, Clone)]
/// Struct containing a single course
pub struct Course {
    pub title: String,
    pub url: String,
    pub tutorials: Vec<Tutorial>,
    pub lang: String,
    /// Sets the course wide tutorial settings (not required)
    pub tutorial_settings: CourseTutorialSettings,
}

#[derive(Debug, Clone)]
/// Course wide defaults for the tutorials (individual tutorials can override some of these)
pub struct CourseTutorialSettings {
    /// This defaults to false
    pub start_closed: bool,
}

impl std::default::Default for CourseTutorialSettings {
    fn default() -> Self {
        CourseTutorialSettings {
            start_closed: false,
        }
    }
}

#[derive(Template, Clone)]
#[template(path = "home.html", escape = "none")]
pub struct Home {
    pub base_url: String,
    pub course_groups: HashMap<String, HashMap<String, Course>>,
}

#[derive(Debug, Clone)]
pub struct Tutorial {
    pub subtitle: String,
    pub content: String,
    /// Defaults to the course wide settings
    pub start_closed: Option<bool>,
}
