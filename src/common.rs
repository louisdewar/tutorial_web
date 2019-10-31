use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::templates::Course;

use serde_yaml::Error;

pub fn display_parse_error(file: &str, err: Error, path: &Path) {
    if let Some(location) = err.location() {
        println!("There was an error parsing the file {:?}", path);
        print!(
            "We think the error occured around line: {}, column {} ",
            location.line(),
            location.column()
        );
        println!(
            "but it is possible the actual error is on a previous line and it was only detected here"
        );

        let line = file.lines().nth(location.line() - 1).unwrap();
        println!("{}", line);
        println!("{: <1$}^^^", "", location.column() - 1);
        println!("Error: {}", format!("{}", err));
    } else {
        println!("Got error: {:?}", err);
    }
}

/// Returns a hashmap of urls to path to course (excluding the .yml)
pub fn get_courses<P: AsRef<Path>>(
    course_folder: P,
    strict_mode: bool,
) -> std::io::Result<HashMap<String, HashMap<String, PathBuf>>> {
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
                    let course = match serde_yaml::from_str::<Course>(&course_str) {
                        Ok(c) => c,
                        Err(e) => {
                            println!("{} ========= Unable to parse: {:?}", if strict_mode { "FATAL" } else { "WARNING" }, &course_path);
                            display_parse_error(&course_str, e, &course_path);

                            if strict_mode {
                                // Exit the program (already printed error don't need to panic)
                                ::std::process::exit(1);
                            } else {
                                // Leave a gap after this error
                                println!("\n");
                                continue;
                            }
                        }
                    };

                    // We want the path up to the name excluding the .yml
                    let path = std::path::Path::new(course_path.parent().unwrap())
                        .join(course_path.file_stem().unwrap());

                    course_groups
                        .entry(course_group_name.clone())
                        .or_insert_with(HashMap::new)
                        .insert(course.url, path);
                }
            }
        }
    }

    Ok(course_groups)
}
