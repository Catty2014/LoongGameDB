mod entity {
    pub mod game;
}
use entity::game;
mod action;
mod login;
mod sonic;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get, post,
    web::{self, Json},
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use config::Config;
use lazy_static::lazy_static;
use sea_orm::Database;
use serde::Deserialize;
use serde_derive::Serialize;

lazy_static! {
    static ref settings: Config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name(".secret.toml"))
        .build()
        .unwrap();
}

#[derive(Serialize)]
struct Response {
    code: u32,
    message: String,
    version: Option<String>,
}

#[get("/")]
async fn hello(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        HttpResponse::Ok().body(format!("Hello {}!", user.id().unwrap()))
    } else {
        HttpResponse::Ok().body("Hello anonymous user!")
    }
}

#[get("/version")]
async fn version() -> impl Responder {
    let response = Response {
        code: 200,
        message: "OK".to_string(),
        version: Some(settings.get_string("VERSION").unwrap()),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let secret = settings.get_string("ACTIX_SECRET").unwrap();
    let secret = Key::from(secret.as_bytes());
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap()).await;
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret.clone(),
            ))
            .service(hello)
            .service(version)
            .service(login::oauth_login)
            .service(login::oauth_callback)
            .service(login::logout)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
