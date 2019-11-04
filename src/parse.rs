use crate::templates::{Course, CourseTutorialSettings, Tutorial};

// use serde_yaml::Value;
use yaml_rust::{yaml::Hash, ScanError, Yaml, YamlLoader};

#[macro_use]
mod macros;

/// Represents the errors that could occur parsing a YAML string into a course struct
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Generic YAML parse error (e.g. not valid YAML or the file was not key=>value)
    YamlError(String, Option<ScanError>),
    /// The value associated with a key was the wrong type (in a context)
    InvalidType(String, String),
    /// A required key does not exist (in a context)
    MissingRequiredKey(String, String),
    /// An unexpected or invalid (wrong type) key (in a context)
    InvalidKey(String, String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;

        match self {
            YamlError(msg, None) => write!(f, "{}", msg),
            YamlError(msg, Some(scan)) => {
                let marker = scan.marker();
                write!(f, "{} ln:{}, col:{}", msg, marker.line(), marker.col())
            }
            InvalidType(msg, context)
            | InvalidKey(msg, context)
            | MissingRequiredKey(msg, context) => write!(f, "{} in context {}", msg, context),
        }
    }
}

/// Check if any keys are not in the list of required and optional (this does not check that required/option keys exist)
fn check_keys(hash: &Hash, allowed_keys: &[&str], context: &str) -> Result<(), ParseError> {
    for key in hash.keys() {
        match key.as_str() {
            Some(key_val) => {
                if !allowed_keys.contains(&key_val) {
                    return Err(ParseError::InvalidKey(
                        format!(
                            "Received a key `{}`, expected one of {:?}",
                            key_val, allowed_keys
                        ),
                        context.to_string(),
                    ));
                }
            }
            None => {
                return Err(ParseError::InvalidKey(format!(
                    "Received a key `{:?}` which was not a string (text), expected one of {:?} in this context",
                    key, allowed_keys
                ), context.to_string()));
            }
        }
    }

    Ok(())
}

pub fn parse_course(course: &str) -> Result<Course, ParseError> {
    // TODO get location of error
    let mut parsed = YamlLoader::load_from_str(course).map_err(|scan_err| {
        ParseError::YamlError("File was not valid YAML".to_owned(), Some(scan_err))
    })?;

    // If there are more than one documents in the file
    if parsed.len() > 1 {
        return Err(ParseError::YamlError(
            "YAML file had more than one document".to_owned(),
            None,
        ));
    }

    // In case no documents (safer to say != 1 which is what we expect)
    if parsed.len() != 1 {
        return Err(ParseError::YamlError(
            "YAML file must contain exactly one document".to_owned(),
            None,
        ));
    }

    let hash = parsed.remove(0).into_hash().ok_or_else(|| {
        ParseError::YamlError(
            "YAML file was not a mapping (key => value).".to_owned(),
            None,
        )
    })?;

    // Define the current context for error messages
    let context = "root level";

    // Check for unrecognised keys
    check_keys(
        &hash,
        &["title", "lang", "url", "tutorials", "tutorial_settings"],
        context,
    )?;

    use itertools::process_results;

    let title: String = yaml_str!(require: hash, title, context).to_string();
    let url: String = yaml_str!(require: hash, url, context).to_string();

    let lang: String = yaml_str!(hash, lang, context).unwrap_or("").to_string();

    let tutorials: Vec<Tutorial> = process_results(
        yaml_vec!(require: hash, tutorials, context)
            .iter()
            .enumerate()
            .map(|(i, tutorial_value)| {
                // Define the current context for error messages
                let context = &format!("tutorial number `{}`", i + 1);

                let hash = match tutorial_value.as_hash() {
                    Some(hash) => hash,
                    None => {
                        return Err(ParseError::InvalidType(format!(
                            "Expected the all of the elements of the tutorials array to be a hash, instead found {:?}",
                            tutorial_value
                        ), context.to_string()))
                    }
                };

                // Check for unrecognised keys
                check_keys(
                    &hash,
                    &["subtitle", "content", "start_closed"],
                    context
                )?;

                let subtitle = yaml_str!(require: hash, subtitle, context).to_string();
                let markdown = yaml_str!(require: hash, content, context);

                let start_closed = yaml_bool!(hash, start_closed, context);

                // Parse markdown
                let parser = pulldown_cmark::Parser::new(&markdown);

                let mut html = String::new();
                pulldown_cmark::html::push_html(&mut html, parser);

                Ok(Tutorial {
                    subtitle,
                    content: html,
                    start_closed,
                })
            }),
        |iter| iter.collect(),
    )?;

    let tutorial_settings = match yaml_hash!(hash, tutorial_settings, context) {
        Some(settings_hash) => {
            // Define the current context for error messages
            let context = "tutorial_settings";

            let mut settings = CourseTutorialSettings::default();

            // Check for unrecognised keys
            check_keys(&settings_hash, &["start_closed"], context)?;

            settings.start_closed =
                yaml_bool!(settings_hash, start_closed, context).unwrap_or(settings.start_closed);

            settings
        }
        None => CourseTutorialSettings::default(),
    };

    Ok(Course {
        title,
        lang,
        url,
        tutorials,
        tutorial_settings,
    })
}
