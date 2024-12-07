// TODO: use oauth2 to refactor code
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get, post,
    web::{self, Json},
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoginCredential {
    username: String,
    password: String,
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
    // let username = body.username;
    // let password = body.password;

    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}
