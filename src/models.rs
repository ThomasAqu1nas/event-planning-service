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

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub reipient: Uuid,
    pub content: Option<String>,
    pub stat: u8,
    pub creation_dt: chrono::DateTime<Utc>,
    pub sending_dt: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, FromRow, serde::Serialize, serde::Deserialize)]
pub struct Invitation {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub link: Option<String>
}
