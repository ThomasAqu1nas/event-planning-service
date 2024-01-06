use uuid::Uuid;

use crate::{dto::{NewEventDto, UpdateEventDto}, PGPool, models::{Event, Invitation}, errors::MyError, db};

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

pub async fn is_participant(user_id: Uuid, event_id: Uuid, pool: &PGPool) -> bool {
   db::event::is_participant(user_id, event_id, pool).await
}

async fn _create_invitation(event_id: Uuid, recipient: Uuid, pool: &PGPool) -> Result<u64, MyError> {
   let user_exists = db::user::exists_by_id(recipient.clone(), pool).await;
   let event_exists = db::event::exists(event_id.clone(), pool).await;
   if user_exists && event_exists {
      let invitation_id = Uuid::new_v4();
      let invitation_link = create_invitation_link(&event_id);
      let invitation: Invitation = Invitation {
        id: invitation_id,
        event_id,
        user_id: recipient,
        link: Some(invitation_link),
      };
      let res = db::invitations::create(invitation, pool)
         .await;
      match res {
         Ok(val) => Ok(val),
         Err(_) => Err(MyError::InternalError)
      }
   } else {
      Err(MyError::BadClientData)
   }
}

pub async fn create_invitation(event_id: Uuid, recipient: Uuid, pool: &PGPool) -> Result<u64, MyError> {
   if db::event::is_participant(recipient.clone(), event_id.clone(), pool).await {
      _create_invitation(event_id, recipient, pool).await
   } else {
      Err(MyError::AuthError)
   }
}

pub async fn subscribe(event_id: Uuid, user_id: Uuid, pool: &PGPool) -> Result<u64, MyError> {
   let res = db::event::subscribe(event_id, user_id, pool)
   .await;
   match res {
      Ok(rows_affected) => Ok(rows_affected.rows_affected()),
      Err(_) => Err(MyError::InternalError)
   }
}

pub fn create_invitation_link(event_id: &Uuid) -> String {
   format!("http://127.0.0.1:8080/accept-invitaion/{:?}", *event_id)
}
