use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, Postgres, QueryBuilder, Execute};
use uuid::Uuid;

use crate::{models::{Event, User}, PGPool, dto};

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

pub async fn set_fields<'a>(id: Uuid, event_fields: dto::UpdateEventDto, pool: &'a PGPool) -> Result<u64, sqlx::Error> {
    let fields: Option<Vec<(String, String)>> = event_fields.get_values();
    match fields {
        Some(v) => {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "UPDATE events SET "
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

pub async fn filter<'a>(filters: Filter, pool: &PGPool) -> Result<Vec<Event>, sqlx::Error> {
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



