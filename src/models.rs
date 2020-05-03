use super::schema::play_histories;

#[derive(Queryable)]
pub struct PlayHistory {
    pub played_at: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="play_histories"]
pub struct NewPlayHistory<'a> {
    pub played_at: &'a str,
    pub body: &'a str,
}
