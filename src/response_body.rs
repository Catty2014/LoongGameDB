use super::entity::game;
use serde::Serialize;

/// 对于大多数请求的基本响应。
#[derive(Serialize)]
pub struct BasicResponse<'a> {
    pub code: u32,
    pub message: &'a str,
}

/// 对于 OAuth 登录的响应。
#[derive(Serialize)]
pub struct AuthUrlResponse<'a> {
    pub code: u32,
    pub message: &'a str,
    pub url: Option<&'a str>,
}

#[derive(Serialize)]
pub struct SearchResponse<'a> {
    pub code: u32,
    pub message: &'a str,
    pub games: Vec<game::Model>,
}

#[derive(Serialize)]
pub struct InfoResponse<'a> {
    pub code: u32,
    pub message: &'a str,
    pub game: Option<game::Model>,
}

#[derive(Serialize)]
pub struct VersionResponse {
    pub code: u32,
    pub message: String,
    pub version: Option<String>,
}
