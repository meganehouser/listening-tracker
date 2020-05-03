#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate env_logger as logger;

use dotenv::dotenv;
use std::env;
use anyhow::Result;
use chrono::SecondsFormat;
use itertools::Itertools as _;

mod schema;
mod models;
mod database;
mod spotify;

use self::spotify::SpotifyManager;
use self::database::PlayHistoryManager;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    logger::init();

    info!("start");
    let database_url = env::var("DATABASE_URL")?;
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let history_limit: u32 = env::var("HISTORY_LIMIT")?.parse()?;

    let history_manager= PlayHistoryManager::connect(&database_url)?;
    let spotify_manager = SpotifyManager::connect(&client_id, &client_secret).await?;
    let items = spotify_manager.get_played_histories(history_limit).await?;
    for item in items.iter() {
        let played_at = item.played_at.to_rfc3339_opts(SecondsFormat::Millis, true);
        let body = format!("{:?}", item);
        let (history, created) = history_manager.get_or_create(&played_at, &body)?;
        if created {
            let artist = item.track.artists.iter()
                .map(|artist| &artist.name)
                .join("/");
            let track_name = &item.track.name;
            info!("{},{},{}", history.played_at, artist, track_name);
        };
    }

    info!("finish");
    Ok(())
}