use log::info;
use sqlx::{postgres::PgQueryResult, query};
use uuid::Uuid;

use crate::{models::{User, Event}, PGPool, dto};

pub async fn create(user: User, pool: &PGPool) -> Result<PgQueryResult, sqlx::Error> {
    let res: Result<PgQueryResult, sqlx::Error> = sqlx::query_as!(User, "INSERT INTO users (id, username, pwd_hash, email, access_token, refresh_token) 
    VALUES ($1, $2, $3, $4, $5, $6)", user.id, user.username, user.pwd_hash, user.email, user.access_token, user.refresh_token)
    .execute(pool)
    .await;
    match res {
        Ok(v) => Ok(v),
        Err(err) => Err(err)
    }
}

pub async fn get_by_id(id: Uuid, pool: &PGPool) -> Result<User, sqlx::Error> {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(pool)
    .await;
    match res {
        Ok(user) => Ok(user),
        Err(err) => Err(err)
    }
}

pub async fn get_all(pool: &PGPool) -> Result<Vec<User>, sqlx::Error> {
    let res = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(pool)
    .await;
    match res {
        Ok(users) => Ok(users),
        Err(err) => Err(err)  
    }
}

pub async fn exists(username: String, pool: &PGPool) -> bool {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
        .fetch_one(pool)
        .await;
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

pub async fn exists_by_id(user_id: Uuid, pool: &PGPool) -> bool {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await;
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

// /users/{id}/participations
pub async fn get_user_participations(id: Uuid, pool: &PGPool) -> Result<Vec<Event>, sqlx::Error> {
    let res = sqlx::query_as!(
        Event, 
        "SELECT * FROM events WHERE id IN (SELECT event_id FROM participations WHERE user_id = $1)", 
        id
    ).fetch_all(pool)
    .await;
    res
}

pub async fn get_id_by_username(username: String, pool: &PGPool) -> Result<Uuid, sqlx::Error> {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_one(pool)
    .await;    
    match res {
        Ok(user) => Ok(user.id),
        Err(err) => Err(err)
    }
}

pub async fn get_by_username(username: String, pool: &PGPool) -> Result<User, sqlx::Error> {
    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_one(pool)
    .await;    
    match res {
        Ok(user) => Ok(user),
        Err(err) => Err(err)
    }
}

pub async fn get_pwd_hash(id: Uuid, pool: &PGPool) -> Result<String, sqlx::Error> {
    let res = sqlx::query_as!(User, 
        "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(pool)
    .await;
    match res {
        Ok(user) => Ok(user.pwd_hash),
        Err(err) => Err(err)
    }
}

pub async fn set_fields(id: Uuid, user_fields: dto::UpdateUserDto, pool: &PGPool) -> Result<u64, sqlx::Error> {
    let fields = user_fields.get_values();
    if let Some(fields) = fields {
        let mut sql = "UPDATE users SET ".to_string();
        for (i, (key, _)) in fields.iter().enumerate() {
            sql.push_str(&format!("{} = ${}, ", key, i + 1));
        }
        sql.truncate(sql.len() - 2);
        sql.push_str(" WHERE id = $");
        sql.push_str(&(fields.len() + 1).to_string());
        info!("SQL string: {:}", sql);
        let mut query = query(&sql);
        for (_, value) in fields.iter() {
            query = query.bind(value);
        }
        query = query.bind(id);
        let result = query.execute(pool).await?;
        Ok(result.rows_affected())
    } else {
        Ok(0)
    }
}


