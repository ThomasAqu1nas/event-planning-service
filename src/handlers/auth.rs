use actix_web::{Responder, web, HttpResponse, HttpRequest};
use log::{error, info};

use crate::{PGPool, dto::NewUserDto, service};

pub async fn login(req: HttpRequest, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    let response = service::auth::jwt::login(conn, req).await;
    match response {
        Ok(val) => {
            info!("RESPONSE /USER/LOGIN: {:?}", val);
            HttpResponse::Ok().json(val)
        },
        Err(err) => {
            error!("[{:} : {:}] INTERNAL SERVER ERROR: {:?}", file!(), line!(), err);
            HttpResponse::InternalServerError().json(err)
        }
    }
}

pub async fn register(dto: web::Json<NewUserDto>, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    let response = service::user::create(dto.0, conn).await;
    match response {
        Ok(val) => {
            info!("RESPONSE /USER/LOGIN: {:?}", val);
            HttpResponse::Ok().json(val)
        },
        Err(err) => {
            error!("[{:} : {:}] INTERNAL SERVER ERROR: {:?}", file!(), line!(), err);
            HttpResponse::InternalServerError().json("Failed to create new user")
        }
    }
}
