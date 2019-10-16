use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use askama::Template;

use crate::templates::Course;

pub fn build_html<P: AsRef<Path>>(input: P, static_files: P, output: P) -> io::Result<()> {
    let mut course_paths: HashMap<String, Vec<Course>> = HashMap::new();

    for course_group_entry in fs::read_dir(input)? {
        let course_group_folder = course_group_entry?.path();
        if course_group_folder.is_dir() {
            let course_group_name = course_group_folder
                .file_name()
                .expect("Couldn't extract course group from folder")
                .to_os_string()
                .into_string()
                .unwrap();

            for tutorial_entry in fs::read_dir(course_group_folder)? {
                let tutorial_path = tutorial_entry?.path();

                if let Some("yml") = tutorial_path.extension().and_then(std::ffi::OsStr::to_str) {
                    let course_str = std::fs::read_to_string(tutorial_path)
                        .expect("Couldn't open and read course file");
                    let course = serde_yaml::from_str::<Course>(&course_str)
                        .expect("Couldn't parse yaml file");

                    course_paths
                        .entry(course_group_name.clone())
                        .or_insert_with(|| vec![])
                        .push(course);
                }
            }
        }
    }

    // Delete existing output files
    if output.as_ref().is_dir() {
        fs::remove_dir_all(output.as_ref())?;
    }

    let course_dir = output.as_ref().join("course");

    for (course_group_name, courses) in course_paths {
        let course_group_dir = course_dir.join(course_group_name);

        fs::create_dir_all(course_group_dir.clone())?;

        for course in courses {
            let html = course.render().expect("Couldn't render course");

            let mut file = fs::File::create(course_group_dir.join(format!("{}.html", course.url)))?;
            file.write_all(html.as_bytes())?;
        }
    }

    use walkdir::WalkDir;

    let static_output = output.as_ref().join("static");

    fs::create_dir_all(static_output.clone())?;

    for entry in WalkDir::new(static_files.as_ref()) {
        let path = entry?.path().to_owned();

        if path.is_file() {
            let rel_path = path
                .strip_prefix(static_files.as_ref())
                .expect("Couldn't get relative path");

            // Copy files from input static to output
            std::fs::copy(path.clone(), static_output.join(rel_path))?;
        }
    }

    println!("Built to {:?}", output.as_ref());

    Ok(())
}
