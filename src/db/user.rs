use sqlx::{postgres::PgQueryResult, QueryBuilder, Postgres, Execute};
use uuid::Uuid;

use crate::{models::{User, Event}, PGPool, dto};

pub async fn create(user: User, pool: &PGPool) -> Result<PgQueryResult, sqlx::Error> {
    let res = sqlx::query_as!(User, "INSERT INTO users (id, username, pwd_hash, email, access_token, refresh_token) 
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

pub async fn set_fields<'a>(id: Uuid, user_fields: dto::UpdateUserDto, pool: &'a PGPool) -> Result<u64, sqlx::Error> {
    let fields: Option<Vec<(String, String)>> = user_fields.get_values();
    match fields {
        Some(v) => {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "UPDATE users SET "
            );
            let mut separated = query_builder.separated(", ");
            for field in v {
                separated.push_bind(format!("{:?} = {:?}", field.0, field.1));
            }
            separated.push_unseparated(format!("WHERE id = {id}"));

            let query = query_builder.build();
            let sql = query.sql();
            println!("function 'set_fields' was executed with sql query string '{:}'", sql);
            let res = sqlx::query(sql)
                .execute(pool)
                .await;
            match res {
                Ok(val) => Ok(val.rows_affected()),
                Err(err) => Err(err)
            }
        },
        None => Ok(0u64)
        
    }
}

// pub async fn check(id: Uuid, current_timestamp: usize, pool: &PGPool) -> Result<bool, sqlx::Error> {
//     let res = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
//         .fetch_one(pool)
//         .await;
//     match res {
//         Ok(user) => {
//             if
//         }
//     }
// }

