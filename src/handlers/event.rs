use actix_web::{Responder, web, get, post, put, HttpResponse, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::{PGPool, service::{auth::UserAuthData, self}, dto::{NewEventDto, UpdateEventDto}, errors::MyError};


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
               HttpResponse::Ok().json(response)
            }, 
            Err(err) => HttpResponse::InternalServerError().json(err)
         }
      }, 
      None => {
         HttpResponse::InternalServerError().json(MyError::Unauthorized)
      }
   }
}

#[post("/{id}/subscribe")]
pub async fn subscribe(req: HttpRequest, event_id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let res = service::event::subscribe(
            event_id.into_inner(), 
            user_auth_data.user_id, 
            conn
         ).await;
         match res {
            Ok(val)  => HttpResponse::Ok().json(val),
            Err(err) => HttpResponse::InternalServerError().json(err)
         }
      },
      None => HttpResponse::from_error(MyError::AuthError)
   }
}

#[get("/")]
pub async fn get_all(pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let res = service::event::get_all(conn)
      .await;
   match res {
      Ok(events) => {
         HttpResponse::Ok().json(events)
      },
      Err(err) => {
         HttpResponse::InternalServerError().json(err)
      }
   }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn: &PGPool = pool_state.get_ref();
   let res = service::event::get_by_id(id.into_inner(), conn)
      .await;
   match res {
      Ok(event) => HttpResponse::Ok().json(event),
      Err(err) => HttpResponse::InternalServerError().json(err)
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
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let update_res = service::event::update(
            id.into_inner(), 
            event_fields, 
            user_auth_data, 
            conn
         ).await;
         match update_res {
            Ok(_) => HttpResponse::Ok().json("Update successfull"),
            Err(err) => HttpResponse::InternalServerError().json(err)
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
   let res = service::event::create_invitation(
      event_id.into_inner(), 
      recipient.into_inner(),
      conn
   ).await;
   match res {
      Ok(_) => HttpResponse::Created().json("invitation created"),
      Err(err) => HttpResponse::InternalServerError().json(err)
   }
}

#[get("/{id}/accept-invitation")]
pub async fn accept_invitation(req: HttpRequest, event_id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
   let conn = pool_state.get_ref();
   match req.extensions().get::<UserAuthData>() {
      Some(user_auth_data) => {
         let recipient = user_auth_data.user_id;
         let res = service::event::subscribe(event_id.into_inner(), recipient, conn)
            .await;
         match res {
            Ok(_) => HttpResponse::Ok().json("invitation accepted"),
            Err(err) => HttpResponse::from_error(err)
         }
      },
      None => HttpResponse::from_error(MyError::AuthError)
   }

}



pub fn init_routes_with_auth(cfg: &mut web::ServiceConfig) {
   cfg.service(create);
   cfg.service(update);
}

pub fn init_public_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(get_by_id);
   cfg.service(get_all);
}