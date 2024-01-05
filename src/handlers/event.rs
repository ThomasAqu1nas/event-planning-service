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

pub fn init_routes_with_auth(cfg: &mut web::ServiceConfig) {
   cfg.service(create);
   cfg.service(update);
}

pub fn init_public_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(get_by_id);
   cfg.service(get_all);
}