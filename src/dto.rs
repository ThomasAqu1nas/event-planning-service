use serde::{Deserialize, Serialize};
use chrono::{self, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct NewUserDto {
    pub username: String,
    pub email: Option<String>,
    pub pwd: String,
    pub pwd_confirm: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewEventDto {
    pub title: String,
    pub descr: String,
    pub dt: chrono::DateTime<chrono::Utc>,
    pub place: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUserRequest {
    pub username: String,
    pub pwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserRespone(pub String);

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub username: String,
    pub exp: usize
}

impl Claims {
    pub fn new(user_id: &Uuid,  username: &String, exp: usize) -> Self {
        Self {
            user_id: *user_id,
            username: username.to_string(),
            exp
        }
    }
}
#[derive(Clone)]
pub struct UpdateUserDto {
    pub pwd_hash: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl UpdateUserDto {
    pub fn get_values(&self) -> Option<Vec<(String, String)>> {
        let mut fields: Vec<(String, String)> = Vec::new();
        if let Some(v) = &self.pwd_hash {
            fields.push(("pwd_hash".to_string(), v.to_string()));
        }
        if let Some(v) = &self.username {
            fields.push(("username".to_string(), v.to_string()));
        }
        if let Some(v) = &self.email {
            fields.push(("email".to_string(), v.to_string()));
        }
        if let Some(v) = &self.access_token {
            fields.push(("access_token".to_string(), v.to_string()));
        }
        if let Some(v) = &self.refresh_token {
            fields.push(("refresh_token".to_string(), v.to_string()));
        }

        if fields.is_empty() {
            None
        } else {
            Some(fields)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEventDto {
    pub title: Option<String>,
    pub descr: Option<String>,
    pub dt: Option<chrono::DateTime<Utc>>,
    pub place: Option<String>,
}

impl UpdateEventDto {
    pub fn get_values(&self) -> Option<Vec<(String, String)>> {
        let mut fields: Vec<(String, String)> = Vec::new();
        if let Some(v) = &self.title {
            fields.push(("title".to_string(), v.to_string()));
        }
        if let Some(v) = &self.descr {
            fields.push(("descr".to_string(), v.to_string()));
        }
        if let Some(v) = &self.dt {
            fields.push(("dt".to_string(), v.timestamp_micros().to_string()));
        }
        if let Some(v) = &self.place {
            fields.push(("place".to_string(), v.to_string()));
        }

        if fields.is_empty() {
            None
        } else {
            Some(fields)
        }
    }
}