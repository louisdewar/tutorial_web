use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use askama::Template;

use crate::templates::{Course, Home, Page};

fn copy_dir(input: &Path, output: &Path) -> io::Result<()> {
    use walkdir::WalkDir;

    fs::create_dir_all(output)?;

    for entry in WalkDir::new(input) {
        let path = entry?.path().to_owned();

        if path.is_file() {
            let rel_path = path
                .strip_prefix(input)
                .expect("Couldn't get relative path");

            // Copy files from input static to output
            std::fs::copy(path.clone(), output.join(rel_path))?;
        }
    }

    Ok(())
}

pub fn build_html<P: AsRef<Path>>(
    input: P,
    static_files: P,
    output: P,
    base_url: String,
) -> io::Result<()> {
    let course_groups_paths = crate::common::get_courses(input)?;

    // Delete existing output files
    if output.as_ref().is_dir() {
        fs::remove_dir_all(output.as_ref())?;
    }

    // Create empty folder
    fs::create_dir_all(output.as_ref())?;

    // ==Handle courses==
    let course_dir = output.as_ref().join("course");

    // This is used for the home page
    let mut course_index = HashMap::new();

    for (course_group_name, courses) in course_groups_paths {
        for (course_name, course_path) in courses {
            let rel_path = format!("{}/{}", course_group_name, course_name);

            fs::create_dir_all(course_dir.join(rel_path.clone()))?;

            let course_str = std::fs::read_to_string(course_path.with_extension("yml"))
                .expect("Couldn't open and read course file");
            let course =
                serde_yaml::from_str::<Course>(&course_str).expect("Couldn't parse yaml file");

            // Append this course to the index
            course_index
                .entry(course_group_name.clone())
                .or_insert_with(HashMap::new)
                .insert(course_name, course.clone());

            let page = Page {
                base_url: base_url.clone(),
                course,
            };

            let html = page.render().expect("Couldn't render course");

            let mut file = fs::File::create(course_dir.join(format!("{}/index.html", rel_path)))?;
            file.write_all(html.as_bytes())?;

            // This directory is the assets folder
            if course_path.is_dir() {
                copy_dir(
                    &course_path,
                    &course_dir.join(format!("{}/assets", rel_path)),
                )?;
            }
        }
    }

    // ==Handle static files==
    let static_output = output.as_ref().join("static");

    copy_dir(static_files.as_ref(), &static_output)?;

    // ==Handle home page==

    let home = Home {
        base_url: base_url.clone(),
        course_groups: course_index,
    };

    let html = home.render().expect("Couldn't render home page");

    let mut index = fs::File::create(output.as_ref().join("index.html"))?;
    index.write_all(html.as_bytes())?;

    // Done
    println!("Built to {:?}", output.as_ref());

    Ok(())
}
