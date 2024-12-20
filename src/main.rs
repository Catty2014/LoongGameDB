mod entity {
    pub mod game;
}
use entity::game;
// mod sonic;
mod login;
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
struct Test {
    code: i32,
    message: String,
}

#[get("/")]
async fn hello(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        HttpResponse::Ok().body(format!("Hello {}!", user.id().unwrap()))
    } else {
        HttpResponse::Ok().body("Hello anonymous user!")
    }
    // let _data = Test {
    //     code: 200,
    //     message: "Hello!".to_string(),
    // };
    // HttpResponse::Ok().json(_data)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .service(login::oauth_login)
            .service(login::oauth_callback)
            // .service(login::index)
            // .service(login::login)
            // .service(login::logout)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
