use sqlx;
use uuid::Uuid;

use crate::{PGPool, models::Invitation};

pub async fn create(
   event_id: Uuid, 
   user_id: Uuid, 
   pool: &PGPool
) -> Result<u64, sqlx::Error>{
   let user_exists = super::user::exists_by_id(user_id.clone(), pool).await;
   let event_exists = super::event::exists(event_id.clone(), pool).await;
   let link = create_invitation_link(&event_id);
   let invitation_id = Uuid::new_v4();
   let invitation = Invitation {
    id: invitation_id,
    event_id,
    user_id,
    link: Some(link),
   };
   if user_exists && event_exists {
      let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query_as!(
         Invitation, 
         "INSERT INTO invitations (id, event_id, user_id, link) 
         VALUES ($1, $2, $3, $4)",
         invitation.id, invitation.event_id, invitation.user_id, invitation.link
      ).execute(pool)
      .await;
      let notification_id = notifications::create(&invitation, pool)
      .await?;
      notifications::update_status_send(&notification_id, pool).await?;
      match res {
         Ok(rows_affected) => Ok(rows_affected.rows_affected()),
         Err(err) => Err(err)
      }

   } else {
      return Err(sqlx::Error::RowNotFound)
   }
}

pub fn create_invitation_link(event_id: &Uuid) -> String {
   format!("http://127.0.0.1:8080/accept-invitaion/{:?}", *event_id)
}

pub mod notifications {
   use crate::{models::Invitation, PGPool};
   use sqlx::postgres::PgQueryResult;
use uuid::Uuid;
   use chrono::Utc;
   pub async fn create(invitaion: &Invitation, pool: &PGPool) -> Result<Uuid, sqlx::Error> {
      let Invitation{id, event_id, user_id, link} = invitaion;
      let notification_id = Uuid::new_v4();
      let content: &str = &format!(
         "#{:?}\n
         You were invited to the event #{:?}\n
         your invitation link: {:?}", 
         *id,
         *event_id, 
         link.clone().unwrap()
      );
      let res = sqlx::query_as!(
         Notification,
         "INSERT INTO notifications (id, recipient, content, stat, creation_dt)
         VALUES ($1, $2, $3, $4, $5)",
         notification_id.clone(),
         *user_id,
         content,
         0,
         Utc::now(),
      ).execute(pool)
      .await;
      match res {
         Ok(_) => Ok(notification_id),
         Err(err) => Err(err)
      }
   }
   pub async fn update_status_send(notification_id: &Uuid, pool: &PGPool) -> Result<PgQueryResult, sqlx::Error> {
      let res = sqlx::query_as!(
         Notification,
         "UPDATE notifications
         SET sending_dt = $1
         WHERE id = $2",
         Utc::now(),
         notification_id
      ).execute(pool)
      .await;
      res
   }
}