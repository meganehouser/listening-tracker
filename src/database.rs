use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use anyhow::{Result, bail};

use super::models::{PlayHistory, NewPlayHistory};

pub struct PlayHistoryManager {
    conn: SqliteConnection,
}

impl PlayHistoryManager {
    pub fn connect(database_url: &str) -> Result<PlayHistoryManager> {
        let conn = SqliteConnection::establish(database_url)?;
        Ok(PlayHistoryManager { conn })
    }

    fn find(&self, dt_iso8601: &str) -> Result<Option<PlayHistory>> {
        use super::schema::play_histories::dsl::*;

        let history = play_histories
            .find(dt_iso8601)
            .first::<PlayHistory>(&self.conn)
            .optional()?;

        Ok(history)
    }

    fn create(&self, dt_iso8601: &str, body: &str) -> Result<()> {
        use super::schema::play_histories;

        let new_play_history = NewPlayHistory {
            played_at: dt_iso8601,
            body,
        };

        diesel::insert_into(play_histories::table)
            .values(&new_play_history)
            .execute(&self.conn)?;

        Ok(())
    }

    pub fn get_or_create(&self, dt_iso8601: &str, body: &str) -> Result<(PlayHistory, bool)> {
        if let Some(history) = self.find(dt_iso8601)? {
            return Ok((history, false));
        };

        self.create(dt_iso8601, body)?;

        if let Some(history) = self.find(dt_iso8601)? {
            return Ok((history, true));
        };

        bail!("Error get_or_create")
    }
}
