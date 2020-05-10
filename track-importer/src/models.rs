//! All kinds of play object
use chrono::prelude::*;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

use rspotify::model::context::Context;
use rspotify::model::artist::SimplifiedArtist;
use rspotify::senum::Type;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    pub duration_ms: i64,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: i64,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayHistory {
    pub track: SimplifiedTrack,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}
