mod games;
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

#[derive(Debug, Deserialize)]
struct LoginCredential {
    username: String,
    password: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let _data = Test {
        code: 200,
        message: "Hello!".to_string(),
    };
    HttpResponse::Ok().json(_data)
}

#[get("/greeting")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[post("/login")]
async fn login(request: HttpRequest, body: Json<LoginCredential>) -> impl Responder {
    // Some kind of authentication should happen here
    // e.g. password-based, biometric, etc.
    // [...]
    dbg!(&request);
    dbg!(&body);
    let username = body.username;
    let password = body.password;

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(index)
            .service(login)
            .service(logout)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
