use uuid::Uuid;

use crate::{dto::{NewEventDto, UpdateEventDto}, PGPool, models::Event, errors::MyError, db};

use super::auth::UserAuthData;

pub async fn create(user_auth_data: &UserAuthData, dto: NewEventDto, pool: &PGPool) -> Result<u64, MyError> {
   let event = Event {
    id: uuid::Uuid::new_v4(),
    title: dto.title,
    descr: dto.descr,
    dt: dto.dt,
    place: dto.place,
    creator: user_auth_data.user_id,
   };
   let res = db::event::create(event, pool)
      .await;
   match res {
      Ok(pg_query_result) => {
         Ok(pg_query_result.rows_affected())
      },
      Err(_) => {
         Err(MyError::InternalError)
      }
   }
}

pub async fn get_all(pool: &PGPool) -> Result<Vec<Event>, MyError> {
   let res = db::event::get_all(pool)
      .await;
   match res {
      Ok(events) => {
         Ok(events)
      },
      Err(_) => Err(MyError::InternalError)
   }
}

pub async fn update(
   id: Uuid, 
   event_fields: UpdateEventDto, 
   user_auth_data: &UserAuthData, 
   pool: &PGPool
) -> Result<u64, MyError> {
   let event_res = db::event::get_by_id(id, pool)
      .await;
   match event_res {
      Ok(event) => {
         if user_auth_data.user_id == event.creator {
            let update_res = db::event::set_fields(
               id, 
               event_fields, 
               pool
            ).await;
            match update_res {
               Ok(rows_affected) => Ok(rows_affected),
               Err(_) => Err(MyError::InternalError)
            }
         } else {
            Err(MyError::Unauthorized)
         }
      }
      Err(_) => Err(MyError::InternalError),
   }
}

pub async fn get_by_id(id: Uuid, pool: &PGPool) -> Result<Event, MyError> {
   let res = db::event::get_by_id(id, pool)
      .await;
   match res {
      Ok(event) => Ok(event),
      Err(_) => Err(MyError::InternalError),
   }      
}

// pub async fn create(dto: NewEventDto, pool: &PGPool, auth_state: Arc<Mutex<UserAuthState>>) -> Result<u64, MyError> {
//     let NewEventDto { title, descr, dt, place } = dto;
//     let res = db::event::create(Event {
//         id: Uuid::new_v4(), title, descr, dt, place,
//         creator: auth_state.lock().unwrap().id.unwrap(),   
//     }, pool).await;  
//     match res {
//         Ok(val) => Ok(val.rows_affected()),
//         Err(_) => Err(MyError::InternalError)
//     }
// }