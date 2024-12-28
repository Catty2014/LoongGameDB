use crate::entity::game::Entity;
use crate::response_body::InfoResponse;
use crate::response_body::SearchResponse;

use super::game;
use super::response_body::BasicResponse;
use super::response_code::ResponseCode;
use super::sonic;
use actix_web::web::Json;
use actix_web::web::Query;
use actix_web::{get, post, HttpResponse};
use config::Config;
use futures::future::join_all;
use lazy_static::lazy_static;
use sea_orm::ActiveModelTrait;
use sea_orm::Database;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;

lazy_static! {
    static ref settings: Config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name(".secret.toml"))
        .build()
        .unwrap();
}

#[derive(Debug, serde::Deserialize)]
pub struct GameIDQuery {
    pub gameid: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct GameNameQuery {
    pub gamename: String,
}

#[get("/info")]
pub async fn info(query: Query<GameIDQuery>) -> HttpResponse {
    // TODO: Reuse db connection
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap()).await;
    if db.is_err() {
        let message = format!("Failed to connect to database: {}", db.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }
    let db = db.unwrap();
    let gameid = query.gameid;
    let game = Entity::find_by_id(gameid).one(&db).await;
    dbg!(&game);
    let response = InfoResponse {
        code: ResponseCode::Success.into(),
        message: "OK",
        game: game.unwrap(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/search")]
pub async fn search(query: Query<GameNameQuery>) -> HttpResponse {
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap()).await;
    if db.is_err() {
        let message = format!("Failed to connect to database: {}", db.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }
    let db = db.unwrap();
    let gamename = query.gamename.to_string();
    let games = sonic::sonic_read_game(gamename);
    if games.is_err() {
        let message = format!("Failed to search game: {}", games.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }
    dbg!(&games);
    let games = join_all(
        games
            .unwrap()
            .iter()
            .map(|id| async { game::Entity::find_by_id(*id).one(&db).await.unwrap() }),
    )
    .await;
    let games: Vec<game::Model> = games.into_iter().map(|game| game.unwrap()).collect();
    dbg!(&games);
    let response = SearchResponse {
        code: ResponseCode::Success.into(),
        message: "OK",
        games,
    };
    HttpResponse::Ok().json(response)
}

#[post("/add")]
pub async fn add(data: Json<game::Model>) -> HttpResponse {
    dbg!(&data);
    let data_active = data.clone().into_active_model();
    dbg!(&data_active);
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap()).await;
    if db.is_err() {
        let message = format!("Failed to connect to database: {}", db.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }
    let db = db.unwrap();
    let result = data_active.insert(&db).await;
    dbg!(&result);
    if result.is_err() {
        let message = format!("Failed to insert game: {}", result.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }
    let result = sonic::sonic_write_game(data.clone());
    dbg!(&result);
    if result.is_err() {
        let message = format!("Failed to insert game: {}", result.err().unwrap());
        let response = BasicResponse {
            code: ResponseCode::DatabaseConnectionError.into(),
            message: message.as_str(),
        };
        return HttpResponse::BadRequest().json(response);
    }

    let response = BasicResponse {
        code: ResponseCode::Success.into(),
        message: "OK",
    };
    HttpResponse::Ok().json(response)
}

#[post("/delete")] // 软删除?
pub async fn delete(query: Query<GameIDQuery>) -> HttpResponse {
    let gameid = query.gameid;
    let game = Entity::find_by_id(gameid);
    dbg!(&game);
    // HttpResponse::Ok().body("dummy")
    let response = BasicResponse {
        code: ResponseCode::NotImplemented.into(),
        message: "Not implemented.",
    };
    HttpResponse::NotImplemented().json(response)
}
