use actix_web::{Responder, web, get, post, put, HttpResponse, HttpRequest, HttpMessage};
use log::{info, error};
use uuid::Uuid;
use crate::{PGPool, service::{auth::UserAuthData, self}, dto::{NewEventDto, UpdateEventDto}, errors::MyError};

#[get("/")]
pub async fn get_all(pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let res = service::event::get_all(conn)
      .await;
   match res {
      Ok(events) => {
         info!("RESPONSE EVENT/: events");
         HttpResponse::Ok().json(events)
      },
      Err(err) => {
         error!("INTERNAL SERVER ERROR: {:?}", err);
         HttpResponse::InternalServerError().json(err)
      }
   }
}

#[post("/create")]
pub async fn create(req: HttpRequest, new_event_dto: web::Json<NewEventDto>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let new_event = new_event_dto.into_inner();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let response_result = service::event::create(user_auth_data, new_event, conn)
         .await;
         match response_result {
            Ok(response) => {
               info!("RESPONSE EVENT/CREATE: {response}");
               HttpResponse::Ok().json(response)
            }, 
            Err(err) => {
               error!("INTERNAL SERVER ERROR: {:?}", err);
               HttpResponse::InternalServerError().json(err)
            }
         }
      }, 
      None => {
         error!("INTERNAL SERVER ERROR: {:?}", MyError::Unauthorized);
         HttpResponse::InternalServerError().json(MyError::Unauthorized)
      }
   }
}

#[post("/{id}/subscribe")]
pub async fn subscribe(req: HttpRequest, event_id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let id: Uuid = event_id.into_inner();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let res = service::event::subscribe(
            id.clone(), 
            user_auth_data.user_id, 
            conn
         ).await;
         match res {
            Ok(val)  => {
               info!("RESPONSE EVENT/{:?}/SUBSCRIBE: {val}", id);
               HttpResponse::Ok().json(val)
            },
            Err(err) => {
               error!("INTERNAL SERVER ERROR: {:?}", err);
               HttpResponse::InternalServerError().json(err)
            }
         }
      },
      None => {
         error!("INTERNAL SERVER ERROR: {:?}", MyError::AuthError);
         HttpResponse::from_error(MyError::AuthError)
      }
   }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let event_id = id.into_inner();
   let res = service::event::get_by_id(event_id.clone(), conn)
      .await;
   match res {
      Ok(event) => {
         info!("RESPONSE EVENT/{:?}: {:?}", event_id, event);
         HttpResponse::Ok().json(event)
      }
      Err(err) => {
         error!("INTERNAL SERVER ERROR: {:?}", err);
         HttpResponse::InternalServerError().json(err)
      }
   }
}

#[put("/update/{id}")]
pub async fn update(
   id: web::Path<Uuid>, 
   update_event_dto: web::Json<UpdateEventDto>, 
   req: HttpRequest,
   pool_state: web::Data<PGPool>
) -> impl Responder {
   let conn = pool_state.get_ref();
   let event_fields = update_event_dto.into_inner();
   let event_id = id.into_inner();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let update_res = service::event::update(
            event_id.clone(), 
            event_fields, 
            user_auth_data, 
            conn
         ).await;
         match update_res {
            Ok(_) => {
               info!("RESPONSE EVENT/UPDATE/{:?}: Update successfull", event_id);
               HttpResponse::Ok().json("Update successfull")
            }
            Err(err) => {
               error!("INTERNAL SERVER ERROR: {:?}", err);
               HttpResponse::InternalServerError().json(err)
            }
         }
      }, 
      None => todo!()
   }
}

#[post("/{id}/invitation")]
pub async fn create_invitation(
   event_id: web::Path<Uuid>,
   recipient: web::Query<Uuid>,
   pool_state: web::Data<PGPool>
) -> impl Responder {
   let conn = pool_state.get_ref();
   let id = event_id.into_inner();
   let res = service::event::create_invitation(
      id.clone(), 
      recipient.into_inner(),
      conn
   ).await;
   match res {
      Ok(_) => {
         info!("RESPONSE EVENT/{:?}/INVITAION: Invitaion created", id);
         HttpResponse::Created().json("Invitation created")
      }
      Err(err) => {
         error!("INTERNAL SERVER ERROR: {:?}", err);
         HttpResponse::InternalServerError().json(err)
      }
   }
}

#[get("/{id}/accept-invitation")]
pub async fn accept_invitation(req: HttpRequest, event_id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn = pool_state.get_ref();
   let id = event_id.into_inner();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let recipient = user_auth_data.user_id;
         let res = service::event::subscribe(id.clone(), recipient, conn)
            .await;
         match res {
            Ok(_) => {
               info!("RESPONSE EVENT/{:?}/ACCEPT-INVITATION: Invitaion accepted", id);
               HttpResponse::Ok().json("Invitation accepted")
            }
            Err(err) => {
               error!("INTERNAL SERVER ERROR: {:?}", err);
               HttpResponse::from_error(err)
            }
         }
      },
      None => {
         error!("INTERNAL SERVER ERROR: {:?}", MyError::AuthError);
         HttpResponse::from_error(MyError::AuthError)
      }
   }

}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(create)
      .service(update)
      .service(subscribe)
      .service(create_invitation)
      .service(accept_invitation)
      .service(get_all)
      .service(get_by_id);
}
