use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::parse::ParseError;

pub fn generate_error_message(mut output: &mut String, file: &str, err: ParseError) {
    use std::fmt::Write;

    match err {
        ParseError::YamlError(msg, Some(scan)) => {
            let location = scan.marker();

            writeln!(
                &mut output,
                "We think the error occured around line: {}, column {} \
but it is possible the actual error is on a previous line and it was only detected here.",
                location.line(),
                location.col()
            )
            .unwrap();

            let line = file.lines().nth(location.line() - 1).unwrap();
            writeln!(&mut output, "{}", line).unwrap();
            writeln!(&mut output, "{: <1$}^^^", "", location.col()).unwrap();
            writeln!(&mut output, "Error: {} ({})", msg, scan).unwrap();
        }
        _ => {
            // Use the default display
            writeln!(&mut output, "{}", err).unwrap();
        }
    }
}

pub enum CourseError {
    Io(std::io::Error),
    Parse(String),
}

impl From<std::io::Error> for CourseError {
    fn from(err: std::io::Error) -> CourseError {
        CourseError::Io(err)
    }
}

/// Returns a hashmap of urls to path to course (excluding the .yml)
pub fn get_courses<P: AsRef<Path>>(
    course_folder: P,
    strict_mode: bool,
) -> Result<HashMap<String, HashMap<String, PathBuf>>, CourseError> {
    let mut course_groups = HashMap::new();

    for course_group_entry in std::fs::read_dir(course_folder)? {
        let course_group_folder = course_group_entry?.path();
        if course_group_folder.is_dir() {
            let course_group_name = course_group_folder
                .file_name()
                .expect("Couldn't extract course group from folder")
                .to_os_string()
                .into_string()
                .unwrap();

            for course_entry in std::fs::read_dir(course_group_folder)? {
                let course_path = course_entry?.path();

                if let Some("yml") = course_path.extension().and_then(std::ffi::OsStr::to_str) {
                    let course_str = std::fs::read_to_string(course_path.clone())
                        .expect("Couldn't open and read course file");
                    let course = match crate::parse::parse_course(&course_str) {
                        Ok(c) => c,
                        Err(err) => {
                            let mut msg = format!(
                                "{} ========= Unable to parse: {:?}\n",
                                if strict_mode { "FATAL" } else { "WARNING" },
                                &course_path
                            );

                            generate_error_message(&mut msg, &course_str, err);

                            if strict_mode {
                                // Exit the program
                                return Err(CourseError::Parse(msg));
                            } else {
                                // Print the message and continue
                                println!("{}\n", msg);
                                continue;
                            }
                        }
                    };

                    // We want the path up to the name excluding the .yml
                    let path = std::path::Path::new(course_path.parent().unwrap())
                        .join(course_path.file_stem().unwrap());

                    // If a course already existed
                    if course_groups
                        .entry(course_group_name.clone())
                        .or_insert_with(HashMap::new)
                        .insert(course.url.clone(), path)
                        .is_some()
                    {
                        return Err(CourseError::Parse(format!(
                            "Two courses (in the same group) had the same url value of `{}/{}`",
                            course_group_name, course.url
                        )));
                    }
                }
            }
        }
    }

    Ok(course_groups)
}
