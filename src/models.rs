use chrono::Utc;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub pwd_hash: String,
    pub username: String,
    pub email: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>
}

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub descr: String,
    pub dt: chrono::DateTime<Utc>,
    pub place: Option<String>,
    pub creator: Uuid
}
