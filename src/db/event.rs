use chrono::{DateTime, Utc};
use log::info;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{models::{Event, User, Participation}, PGPool, dto};

pub enum Filter{
    DT((DateTime<Utc>, DateTime<Utc>)),
    PLACE(String),
    CREATOR(Uuid)
}

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

pub async fn exists(id: Uuid, pool: &PGPool) -> bool {
    let res = sqlx::query_as!(Event, "SELECT * FROM events WHERE id = $1", id)
        .fetch_one(pool)
        .await;
    match res {
        Ok(_) => true,
        Err(_) => false
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

pub async fn subscribe(event_id: Uuid, user_id: Uuid, pool: &PGPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_as!(
        Participation,
        "INSERT INTO participations (event_id, user_id)
        VALUES ($1, $2)",
        event_id, user_id
    ).execute(pool)
    .await
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

pub async fn is_participant(user_id: Uuid, event_id: Uuid, pool: &PGPool) -> bool {
    let res = sqlx::query_as!(
        Participation,
        "SELECT * FROM participations WHERE event_id = $1 AND user_id = $2",
        event_id, user_id
    ).fetch_one(pool)
    .await;
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

pub async fn set_fields(id: Uuid, event_fields: dto::UpdateEventDto, pool: &PGPool) -> Result<u64, sqlx::Error> {
    let fields = event_fields.get_values();

    if let Some(fields) = fields {
        let mut sql = "UPDATE events SET ".to_string();
        for (i, (key, _)) in fields.iter().enumerate() {
            sql.push_str(&format!("{} = ${}, ", key, i + 1));
        }
        sql.truncate(sql.len() - 2);
        sql.push_str(" WHERE id = $");
        sql.push_str(&(fields.len() + 1).to_string());
        info!("SQL string: {:}", sql);
        let mut query = sqlx::query(&sql);
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

pub async fn filter(filters: Filter, pool: &PGPool) -> Result<Vec<Event>, sqlx::Error> {
    match filters {
        Filter::DT(datetime) => {
            sqlx::query_as!(
                Event,
                "SELECT * FROM events WHERE dt >= $1 AND dt < $2",
                datetime.0,
                datetime.1
            ).fetch_all(pool)
            .await
        },
        Filter::PLACE(place) => {
            sqlx::query_as!(
                Event,
                "SELECT * FROM events WHERE place = $1",
                place
            ).fetch_all(pool)
            .await
        },
        Filter::CREATOR(creator_id) => {
            sqlx::query_as!(
                Event,
                "SELECT * FROM events WHERE creator = $1",
                creator_id
            ).fetch_all(pool)
            .await  
        },
    }
}



