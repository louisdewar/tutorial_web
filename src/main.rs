use askama::Template;
use std::collections::HashMap;

use actix_web::{Either, HttpResponse, middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_files as fs;

mod templates;
use templates::Course;

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn render_course(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let course = req.match_info().get("name").and_then(|course_name| {
        state.courses.get(course_name)
    });

    if let Some(course) = course {
        let result = course.render().unwrap();

        Either::A(HttpResponse::Ok().body(result))
    } else {
        Either::B("Couldn't find course")
    }
}

#[derive(Clone)]
struct AppState {
    pub courses: HashMap<String, Course>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut courses = HashMap::new();

    // Load courses
    for course_path in &["example_course.yml"] {
        let course_str = std::fs::read_to_string(course_path).unwrap();
        let course: Course = serde_yaml::from_str(&course_str).unwrap();

        courses.insert(course.title.clone(), course);
    }

    let app_state = AppState { courses };

    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(app_state.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .service(web::resource("/course/{name}").to(render_course))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}

//
// fn main() {
//     let course_str = std::fs::read_to_string("example_course.yml").unwrap();
//     let course: Course = serde_yaml::from_str(&course_str).unwrap();
//
//     println!("{}", course.render().unwrap());
// }
