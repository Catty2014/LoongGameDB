use super::game;
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

pub fn sonic_connection_test() -> bool {
    let channel = IngestChannel::start(
        settings.get_string("SONICDB_URL").unwrap(),
        settings.get_string("SONICDB_PASSWORD").unwrap(),
    );
    dbg!(&channel);
    channel.is_ok()
}

pub fn sonic_write_game(game: game::Model) -> Result<(), String> {
    // PERFORMANCE: 用r2d2重写该部分以加快效率
    let channel = IngestChannel::start(
        settings.get_string("SONICDB_URL").unwrap(),
        settings.get_string("SONICDB_PASSWORD").unwrap(),
    )
    .unwrap();

    let dest = Dest::col_buc("loonggamedb", "games").obj(game.id);
    let pushed = channel.push(PushRequest::new(dest, game.name));
    dbg!(&pushed);
    if pushed.is_err() {
        return Err(pushed.err().unwrap().to_string());
    }

    Ok(())
}

pub fn sonic_read_game(name: String) -> Result<Vec<u32>, String> {
    // PERFORMANCE: 用r2d2重写该部分以加快效率
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
            .map(|i| i.parse::<u32>().ok().unwrap())
            .collect()),
        Err(error) => Err("Failed while reading from sonic:".to_owned() + &error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use enumflags2::make_bitflags;
    use game::CompatibilityLayerItem;
    // use sea_orm::IntoActiveModel;

    use super::*;

    #[test]
    fn test_sonic_connection_test() {
        assert!(sonic_connection_test());
    }

    #[test]
    fn test_sonic_write_game() {
        let game = game::Model {
            id: 1,
            name: "test".to_owned(),
            supportlevel: game::SupportLevel::GREAT,
            compat: game::Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{LATX}),
            },
        };
        sonic_write_game(game).unwrap();
    }

    #[test]
    fn test_sonic_read_game() {
        let game = game::Model {
            id: 1,
            name: "Test Music 001".to_owned(),
            supportlevel: game::SupportLevel::GREAT,
            compat: game::Compatibility {
                0: make_bitflags!(CompatibilityLayerItem::{LATX}),
            },
        };
        sonic_write_game(game).unwrap();
        let games = sonic_read_game("Test Music 001".to_owned()).unwrap();
        dbg!(&games);
        assert_eq!(games.len(), 1);
        assert_eq!(games[0], 1);
    }
}
