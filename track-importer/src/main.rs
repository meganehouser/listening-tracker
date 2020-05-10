use dotenv::dotenv;
use anyhow::{Context, Result};
use chrono::SecondsFormat;
use std::env;
use rusqlite::{Connection, params};
use mongodb::{Client, options::{ClientOptions, auth::Credential}, Collection};
use bson::{doc};

mod models;

use models::PlayHistory;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let mongo_url = env::var("MONGO_URL").unwrap();
    let mongo_user = env::var("MONGO_USER").unwrap();
    let mongo_password = env::var("MONGO_PASSWORD").unwrap();
    let conn = Connection::open(db_url)?;

    let mut stmt = conn.prepare("SELECT * FROM play_histories")?;
    let mut rows = stmt.query(params![])?;

    let mut client_options = ClientOptions::parse(&mongo_url).await?;
    let mut credential = Credential::default();
    credential.username = Some(mongo_user);
    credential.password = Some(mongo_password);
    client_options.credential = Some(credential);

    let client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("play_histories");
    let raw_histories: Collection = db.collection("raw_histories");


    while let Some(row) = rows.next()? {
        let data: String = row.get(1)?;
        let history: PlayHistory = serde_json::from_str(&data)?;
        let criteria = doc!{"played_at": history.played_at.to_rfc3339_opts(SecondsFormat::Millis, true)};
        if raw_histories.find_one(criteria, None).await?.is_none() {
            let bson = bson::to_bson(&history)?;
            let doc = bson.as_document().context("as document error")?;
            raw_histories.insert_one(doc.to_owned(), None).await?;
        };
    }

    Ok(())
}
