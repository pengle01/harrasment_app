use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct Info {
    username: String,
    logitude: f32,
    latitude: f32,
    message: String,
}

/// Update the position of a user and chech whether the user needs help
async fn update_data(info: web::Json<Info>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Info {
        username: String::from(info.username.as_str()),
        logitude: info.logitude,
        latitude: info.latitude,
        message: String::from(info.message.as_str()),
    }))
}

/// Add a new user in the hashmap. If it exits override
async fn register_user(info: web::Json<Info>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Info {
        username: String::from(info.username.as_str()),
        logitude: info.logitude,
        latitude: info.latitude,
        message: String::from(info.message.as_str()),
    }))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(index))
            .route("/", web::put().to(manage_data))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("192.168.10.2:6666")?
    .run()
    .await
}
