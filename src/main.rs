use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

#[derive(Deserialize, Serialize)]
struct Point {
    logitude: f32,
    latitude: f32,
}

#[derive(Deserialize, Serialize)]
struct UserPoint {
    username: String,
    message: String,
    point: Point,
}

/// Update the position of a user and chech whether the user needs help
async fn update_data(
    info: web::Json<UserPoint>,
    data: web::Data<Mutex<HashMap<String, Point>>>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(UserPoint {
        username: String::from(info.username.as_str()),
        message: String::from(info.message.as_str()),
        point: Point {
            logitude: info.point.logitude,
            latitude: info.point.latitude,
        },
    }))
}

/// Add a new user in the hashmap. If it exits override
async fn register_user(
    info: web::Json<UserPoint>,
    data: web::Data<Mutex<HashMap<String, Point>>>,
) -> Result<HttpResponse> {
    let mut map = data.lock().unwrap();
    map.insert(
        String::from(info.username.as_str()),
        Point {
            logitude: info.point.logitude,
            latitude: info.point.latitude,
        },
    );
    Ok(HttpResponse::Ok().json(UserPoint {
        username: String::from(info.username.as_str()),
        message: String::from(info.message.as_str()),
        point: Point {
            logitude: info.point.logitude,
            latitude: info.point.latitude,
        },
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let shared_state = web::Data::new(Mutex::new(HashMap::<String, Point>::new()));
        App::new()
            .app_data(shared_state.clone())
            .route("/", web::post().to(update_data))
            .route("/", web::put().to(register_user))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("192.168.10.2:6666")?
    .run()
    .await
}
