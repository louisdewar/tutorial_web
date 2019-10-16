use askama::Template;

use actix_files as fs;
use actix_web::{middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder};

use std::collections::HashMap;
use std::path::PathBuf;

use crate::templates::Course;

fn render_course(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    if let (Some(topic), Some(name)) = (req.match_info().get("topic"), req.match_info().get("name"))
    {
        if let Some(path) = state.course_urls.get(&format!("{}/{}", topic, name)) {
            match std::fs::read_to_string(path)
                .map_err(|_| "Couldn't open and read file")
                .and_then(|course_str| {
                    serde_yaml::from_str::<Course>(&course_str)
                        .map_err(|_| "Couldn't parse yaml file")
                })
                .and_then(|course| {
                    course
                        .render()
                        .map_err(|_| "Couldn't render course into html")
                }) {
                Ok(result) => Either::A(HttpResponse::Ok().body(result)),
                Err(msg) => Either::B(msg),
            }
        } else {
            Either::B("The url course wasn't found, if you have recently created the file try restarting the server")
        }
    } else {
        Either::B("Pass in the correct parameters")
    }
}

// Returns a hashmap of urls to path to course
fn get_courses(course_folder: &str) -> std::io::Result<HashMap<String, PathBuf>> {
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

                    urls.insert(
                        format!("{}/{}", course_group_name.clone(), course.url),
                        course_path,
                    );
                }
            }
        }
    }

    Ok(urls)
}

#[derive(Clone)]
struct AppState {
    pub course_urls: HashMap<String, PathBuf>,
}

pub fn start_server(static_folder: String, course_folder: &str) -> std::io::Result<()> {
    let course_urls = get_courses(course_folder)?;

    if course_urls.is_empty() {
        println!("Could find any files");
        return Ok(());
    }

    println!("Loaded the following files:");

    for (i, url) in course_urls.keys().enumerate() {
        println!("{}. http://127.0.0.1:8000/course/{}.html", i + 1, url);
    }

    println!(
        "\nIf you create a new course which isn't listed here you must restart the server to see a change
If you edit a course which is listed here you must simply reload the webpage to see the new version."
);

    println!("Starting webserver at http://127.0.0.1:8000");
    println!("This server is only for local testing, it is not designed to scale well although it should be fast");
    println!("Use the build command to generate the production files and then serve them");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app_state = AppState { course_urls };

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .register_data(web::Data::new(app_state.clone()))
            .service(web::resource("/course/{topic}/{name}.html").to(render_course))
            .service(fs::Files::new("/static", static_folder.clone()).show_files_listing())
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();

    Ok(())
}
