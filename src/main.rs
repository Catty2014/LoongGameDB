use crate::game::Entity;
use crate::response_body::VersionResponse;
use log::{debug, error, info, warn};
use sea_orm::schema;
use sea_orm::ConnectionTrait;
mod entity {
    pub mod game;
}
mod response_body;
mod response_code;
use entity::game;
mod action;
mod login;
mod sonic;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, get, App, HttpResponse, HttpServer, Responder};
use config::Config;
use lazy_static::lazy_static;
use sea_orm::Database;

lazy_static! {
    static ref settings: Config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name(".secret.toml"))
        .build()
        .unwrap();
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
    let response = VersionResponse {
        code: 200,
        message: "OK".to_string(),
        version: Some(settings.get_string("VERSION").unwrap()),
    };
    HttpResponse::Ok().json(response)
}

async fn init() -> Result<(), ()> {
    env_logger::init();
    debug!("Initializing database...");
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let builder = db.get_database_backend();
    let schema = schema::Schema::new(builder);
    let result = db
        .execute(builder.build(&schema.create_table_from_entity(Entity)))
        .await;
    // dbg!(&result);
    match result {
        Ok(_t) => {}
        Err(e) => {
            if e.to_string().contains("already exists") {
                info!("Table already exists, skipping table creation...");
            } else {
                error!("Failed to create table!");
                dbg!(e);
                return Err(());
            }
        }
    };
    debug!("Testing SonicDB connection...");
    if !sonic::sonic_connection_test() {
        warn!("SonicDB connection test failed, skipping...");
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init().await.unwrap();
    let secret = settings.get_string("ACTIX_SECRET").unwrap();
    let secret = Key::from(secret.as_bytes());
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
            .service(action::add)
            .service(action::info)
            .service(action::search)
            .service(action::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
