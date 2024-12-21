use super::game;
use super::sonic;
use actix_web::web::Query;
use actix_web::{get, web, HttpResponse};
use config::Config;
use lazy_static::lazy_static;
use sea_orm::EntityTrait;

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

#[get("/info")]
pub async fn info(query: Query<GameIDQuery>) -> HttpResponse {
    let gameid = query.gameid;
    let game = game::Entity::find_by_id(gameid);
    HttpResponse::Ok().body("dummy")
}
