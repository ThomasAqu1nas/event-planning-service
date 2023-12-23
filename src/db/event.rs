use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{models::{Event, User}, PGPool};


pub async fn create(event: Event, pool: &PGPool) -> Result<PgQueryResult, sqlx::Error> {
    let res = sqlx::query_as!(Event, "INSERT INTO events (id, title, descr, dt, place, creator) 
    VALUES ($1, $2, $3, $4, $5, $6)", 
    event.id, event.title, event.descr, event.dt, event.place, event.creator)
    .execute(pool)
    .await;
    match res {
        Ok(v) => Ok(v),
        Err(err) => Err(err)
    }
}
// /events/id
pub async fn get_by_id(id: Uuid, pool: &PGPool) -> Result<Event, sqlx::Error> {
    let res = sqlx::query_as!(Event, "SELECT * FROM events WHERE id = $1", id)
    .fetch_one(pool)
    .await;
    match res {
        Ok(event) => Ok(event),
        Err(err) => Err(err)
    }
}

pub async fn get_all(pool: &PGPool) -> Result<Vec<Event>, sqlx::Error> {
    let res = sqlx::query_as!(Event, "SELECT * FROM events")
    .fetch_all(pool)
    .await;
    match res {
        Ok(events) => Ok(events),
        Err(err) => Err(err)
    }
}

pub async fn get_participants(id: Uuid, pool: &PGPool) -> Result<Vec<User>, sqlx::Error> {
    let res = sqlx::query_as!(
        User, 
        "SELECT * FROM users WHERE id IN (SELECT event_id FROM participations WHERE event_id = $1)",
        id
    ).fetch_all(pool)
    .await;
    res
}



