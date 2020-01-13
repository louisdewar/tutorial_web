use askama::Template;

use actix_files as fs;
use actix_web::{middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder};

use std::collections::HashMap;
use std::path::PathBuf;

use crate::templates::Page;

fn render_course(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    if let (Some(topic), Some(name)) = (req.match_info().get("topic"), req.match_info().get("name"))
    {
        if let Some(path) = state
            .course_urls
            .get(topic)
            .and_then(|course_groups| course_groups.get(name))
        {
            match std::fs::read_to_string(path.with_extension("yml"))
                .map_err(|_| "Couldn't open and read file".to_string())
                .and_then(|course_str| {
                    crate::parse::parse_course(&course_str)
                        .map_err(|err| format!("Couldn't parse yaml file: {}", err))
                })
                .and_then(|course| {
                    let page = Page {
                        base_url: "".to_string(),
                        course,
                    };

                    page.render()
                        .map_err(|_| "Couldn't render course into html".to_string())
                }) {
                Ok(result) => Either::A(HttpResponse::Ok().body(result)),
                Err(msg) => Either::B(msg),
            }
        } else {
            Either::B("The url course wasn't found, if you have recently created the file try restarting the server".to_string())
        }
    } else {
        Either::B("Pass in the correct parameters".to_string())
    }
}

fn serve_assets(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    // If our routes are setup correctly it should be impossible for this to fail
    let topic = req
        .match_info()
        .get("topic")
        .expect("Missing parameters from routes");
    let name = req
        .match_info()
        .get("name")
        .expect("Missing parameters from routes");
    let asset_path = req
        .match_info()
        .get("asset_path")
        .expect("Missing parameters from routes");

    if let Some(mut path) = state
        .course_urls
        .get(topic)
        .and_then(|course_group| course_group.get(name))
        .cloned()
    {
        // It is likely possible for an attacker to use this to preform a reverse traversal attack
        // another reason why this code should only be used for local testing
        path.push(asset_path);

        match fs::NamedFile::open(path) {
            Ok(file) => Either::A(file),
            Err(_) => Either::B(HttpResponse::NotFound().body("Couldn't find/open the file")),
        }
    } else {
        Either::B(HttpResponse::NotFound().body("The url course wasn't found, if you have recently created the file try restarting the server"))
    }
}

#[derive(Clone)]
struct AppState {
    pub course_urls: HashMap<String, HashMap<String, PathBuf>>,
}

pub fn start_server(static_folder: String, course_folder: &str) -> std::io::Result<()> {
    use crate::common::{get_courses, CourseError};
    // Get courses in a non-strict way (if there is an error just skip)
    let course_urls = match get_courses(course_folder, false) {
        Ok(val) => val,
        Err(CourseError::Io(err)) => return Err(err),
        Err(CourseError::Parse(err)) => {
            println!(
                "Server intialisation failed because it couldn't load course files:\n{}",
                err
            );
            std::process::exit(1);
        }
    };

    if course_urls.is_empty() {
        println!("Could find any files");
        return Ok(());
    }

    println!("Loaded the following files:");

    for (group_name, courses) in &course_urls {
        println!("==={}===", group_name);
        for (i, course_name) in courses.keys().enumerate() {
            println!(
                "{}. http://127.0.0.1:8000/course/{}/{}/index.html",
                i + 1,
                group_name,
                course_name
            );
        }
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
            .register_data(web::Data::new(app_state.clone()))
            .service(web::resource("/course/{topic}/{name}/index.html").to(render_course))
            .service(
                web::resource("/course/{topic}/{name}/assets/{asset_path:.*}").to(serve_assets),
            )
            .service(fs::Files::new("/static", static_folder.clone()).show_files_listing())
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();

    Ok(())
}
