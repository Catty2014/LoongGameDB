use actix_session::Session;
use actix_web::{get, post, web::Query, HttpMessage, Responder};
// TODO: use oauth2 to refactor code
// use anyhow;
use config;
use config::Config;
use lazy_static::lazy_static;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
// use oauth2::reqwest::http_client;
use ::reqwest::Client;
//use reqwest::http_client as reqwest_http_client;
use oauth2::*;
// use sea_orm::Identity;
use serde::{Deserialize, Serialize};
// use url::Url;

lazy_static! {
    static ref settings: Config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name(".secret.toml"))
        .build()
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct AuthUrlResponse<'a> {
    code: u32,
    msg: &'a str,
    url: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse<'a> {
    code: u32,
    msg: &'a str,
}

#[derive(Serialize, Deserialize)]
struct AuthCallback {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResource {
    login: String,
}

// Based on
// https://github.com/ramosbugs/oauth2-rs/blob/9a2b746f76c5d0f9a7a02a1916bd509668fcaee3/examples/github_async.rs
#[get("/login/oauth")]
pub async fn oauth_login(session: Session) -> impl Responder {
    let github_client_id: ClientId =
        ClientId::new(settings.get_string("OAUTH_GITHUB_CLIENT_ID").unwrap());
    let github_client_secret: ClientSecret =
        ClientSecret::new(settings.get_string("OAUTH_GITHUB_CLIENT_SECRET").unwrap());
    let auth_url: AuthUrl =
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap();
    let token_url: TokenUrl =
        TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap();
    let redirect_url =
        RedirectUrl::new(settings.get_string("OAUTH_REDIRECT_URL").unwrap()).unwrap();
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();
    let response = AuthUrlResponse {
        code: 200,
        msg: "Success",
        url: Some(authorize_url.as_str()),
    };
    // dbg!(&authorize_url);
    // dbg!(&csrf_state.secret());
    let result = session.insert("CSRF_state", csrf_state.secret());
    if result.is_err() {
        let response = AuthResponse {
            code: 500,
            msg: "Failed to insert CSRF state.",
        };
        return actix_web::HttpResponse::Ok().json(response);
    }
    actix_web::HttpResponse::Ok().json(response)
}

#[get("/login/callback")]
pub async fn oauth_callback(
    session: Session,
    query: Query<AuthCallback>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    // dbg!(&request);
    // dbg!(&request.query_string());
    // dbg!(&query.code);
    // dbg!(&query.state);
    let code = query.code.clone();
    let state_expected: Option<String> = session.get("CSRF_state").unwrap_or(None);
    match state_expected {
        Some(_state) => {
            if _state != query.state {
                // Verify CSRF state
                let response = AuthResponse {
                    code: 400,
                    msg: "CSRF state verification failed.",
                };
                return actix_web::HttpResponse::Ok().json(response);
            }
        }
        None => {
            let response = AuthResponse {
                code: 400,
                msg: "CSRF state verification failed.",
            };
            return actix_web::HttpResponse::Ok().json(response);
        }
    }
    session.remove("CSRF_state");

    let github_client_id: ClientId =
        ClientId::new(settings.get_string("OAUTH_GITHUB_CLIENT_ID").unwrap());
    let github_client_secret: ClientSecret =
        ClientSecret::new(settings.get_string("OAUTH_GITHUB_CLIENT_SECRET").unwrap());
    let auth_url: AuthUrl =
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap();
    let token_url: TokenUrl =
        TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap();
    let redirect_url =
        RedirectUrl::new(settings.get_string("OAUTH_REDIRECT_URL").unwrap()).unwrap();
    let resource_url = settings.get_string("OAUTH_RESOURCE_URL").unwrap();
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);
    let token_res = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;
    dbg!(&token_res);
    if let Ok(token) = token_res {
        let token = token.access_token();
        dbg!(&token.secret());
        let httpclient = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0")
            .build()
            .unwrap();
        let res = httpclient
            .get(resource_url)
            .bearer_auth(token.secret())
            .send()
            .await;
        dbg!(&res);
        match res {
            Ok(res) => {
                let username = res.json::<AuthResource>().await.unwrap().login;
                dbg!(&username);
                actix_identity::Identity::login(&request.extensions(), format!("{}", username))
                    .unwrap();
                let response = AuthResponse {
                    code: 200,
                    msg: "OK",
                };
                return actix_web::HttpResponse::Ok().json(response);
            }
            Err(err) => {
                let response = AuthResponse {
                    code: 500,
                    msg: "Failed to get username.", // TODO: better error message
                };
                return actix_web::HttpResponse::Ok().json(response);
            }
        }
    } else {
        let response = AuthResponse {
            code: 500,
            msg: "Failed to get token.",
        };
        return actix_web::HttpResponse::Ok().json(response);
    }
}

#[post("/logout")]
async fn logout(user: actix_identity::Identity) -> impl Responder {
    user.logout();
    actix_web::HttpResponse::Ok()
}
