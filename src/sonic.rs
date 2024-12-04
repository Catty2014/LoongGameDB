use super::games::Game;
use config;
use config::Config;
use lazy_static::lazy_static;
use sonic_channel2::Dest;
use sonic_channel2::IngestChannel;
use sonic_channel2::PushRequest;
use sonic_channel2::QueryRequest;
use sonic_channel2::SearchChannel;
use sonic_channel2::SonicChannel;

lazy_static! {
    static ref settings: Config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .add_source(config::File::with_name(".secret.toml"))
        .build()
        .unwrap();
}
pub fn sonic_write_game(game: Game) -> Result<(), ()> {
    // TODO: 用r2d2重写该部分以加快效率
    let channel = IngestChannel::start(
        settings.get_string("SONICDB_URL").unwrap(),
        settings.get_string("SONICDB_PASSWORD").unwrap(),
    )
    .unwrap();

    let dest = Dest::col_buc("loonggamedb", "games").obj(game.id);
    let pushed = channel.push(PushRequest::new(dest, game.name));

    dbg!(pushed);
    Ok(())
}

pub fn sonic_read_game(name: String) -> Result<Vec<u64>, String> {
    // TODO: 用r2d2重写以加快效率
    let channel = SearchChannel::start(
        settings.get_string("SONICDB_URL").unwrap(),
        settings.get_string("SONICDB_PASSWORD").unwrap(),
    )
    .unwrap();

    let game = channel.query(QueryRequest::new(
        Dest::col_buc("loonggamedb", "games"),
        name,
    ));

    dbg!(&game);
    match game {
        Ok(candidates) => Ok(candidates
            .iter()
            .map(|i| i.parse::<u64>().ok().unwrap())
            .collect()),
        Err(error) => Err("Failed while reading from sonic:".to_owned() + &error.to_string()),
    }
}
