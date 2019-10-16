use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::templates::Course;

/// Returns a hashmap of urls to path to course (excluding the .yml)
pub fn get_courses<P: AsRef<Path>>(course_folder: P) -> std::io::Result<HashMap<String, PathBuf>> {
    let mut urls = HashMap::new();

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
                    let course = serde_yaml::from_str::<Course>(&course_str)
                        .expect("Couldn't parse yaml file");

                    // We want the path up to the name excluding the .yml
                    let path = std::path::Path::new(course_path.parent().unwrap())
                        .join(course_path.file_stem().unwrap());

                    urls.insert(
                        format!("{}/{}", course_group_name.clone(), course.url),
                        path,
                    );
                }
            }
        }
    }

    Ok(urls)
}
