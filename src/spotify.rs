use anyhow::{Result, bail};
use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;
use rspotify::model::playing::PlayHistory;


const REDIRECT_URL: &str = "http://localhost:8888/callback";

pub struct SpotifyManager {
    spotify: Spotify,
}

impl SpotifyManager {
    pub async fn connect(client_id: &str, client_secret: &str) -> Result<SpotifyManager> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(client_id)
            .client_secret(client_secret)
            .redirect_uri(REDIRECT_URL)
            .scope("user-read-recently-played")
            .build();

        if let Some(token_info) = get_token(&mut oauth).await {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            return Ok(SpotifyManager {spotify})
        };

        bail!("authentication failed")
    }

    pub async fn get_played_histories(&self, limit: u32) -> Result<Vec<PlayHistory>> {
        match self.spotify.current_user_recently_played(limit).await {
            Ok(history) => Ok(history.items),
            Err(e) => bail!(format!("{}", e)),
        }
    }
}
