use super::game;
use super::sonic;
use actix_web::web::Json;
use actix_web::web::Query;
use actix_web::{get, post, web, HttpResponse};
use config::Config;
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
    let gameid = query.gameid;
    let game = game::Entity::find_by_id(gameid);
    dbg!(&game);
    HttpResponse::Ok().body("dummy")
}

#[get("/search")]
pub async fn search(query: Query<GameNameQuery>) -> HttpResponse {
    let gamename = query.gamename.to_string();
    let games = sonic::sonic_read_game(gamename);
    dbg!(&games);
    HttpResponse::Ok().body("dummy")
}

#[post("/add")]
pub async fn add(data: Json<game::Model>) -> HttpResponse {
    dbg!(&data);
    let data_active = data.clone().into_active_model();
    dbg!(&data_active);
    let db = Database::connect(settings.get_string("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let result = data_active.insert(&db).await;
    dbg!(result);
    let result = sonic::sonic_write_game(data.clone());
    dbg!(result);
    HttpResponse::Ok().body("dummy")
}

#[get("/delete")] // 软删除?
pub async fn delete(query: Query<GameIDQuery>) -> HttpResponse {
    let gameid = query.gameid;
    let game = game::Entity::find_by_id(gameid);
    dbg!(&game);
    HttpResponse::Ok().body("dummy")
}
