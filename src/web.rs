use askama::Template;

use actix_web::{Either, HttpResponse, middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_files as fs;

use crate::templates::Course;

fn render_course(req: HttpRequest) -> impl Responder {
    if let (Some(topic), Some(name)) = (req.match_info().get("topic"), req.match_info().get("name")) {
        use std::path::PathBuf;
        let mut path = PathBuf::from("./courses");
        path.push(topic);
        path.push(format!("{}.yml", name));

        match std::fs::read_to_string(path).map_err(|_| "Couldn't open and read file")
        .and_then(|course_str| {
            serde_yaml::from_str::<Course>(&course_str).map_err(|_| "Couldn't parse yaml file")
        }).and_then(|course| {
            course.render().map_err(|_| "Couldn't render course into html")
        }) {
            Ok(result) => Either::A(HttpResponse::Ok().body(result)),
            Err(msg) => Either::B(msg),
        }
    } else {
        Either::B("Pass in the correct parameters")
    }
}

pub fn start_server() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/course/{topic}/{name}").to(render_course))
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
