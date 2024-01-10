use actix_web::{Responder, web, get, HttpResponse};
use uuid::Uuid;

use crate::PGPool;
use crate::service;

#[get("/")]
pub async fn get_all(pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    println!("{:?}", "GET ALL");
    let response = service::user::get_all(conn).await;
    match response {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}

#[get("/{id}")]
pub async fn get_by_id(id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    let user_id = id.into_inner();
    println!("GET BY ID: {:?} ", user_id);
    let response = service::user::get_by_id(user_id, conn).await;
    match response {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}

#[get("/{id}/participations")]
pub async fn get_user_participations(id: web::Path<Uuid>, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    let user_id = id.into_inner();
    println!("GET PARTICIPATIONS BY ID: {:?}", user_id);
    let response = service::user::get_user_participations(user_id, conn)
        .await;
    match response {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(err) => HttpResponse::InternalServerError().json(err)
    } 
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(get_by_id)
        .service(get_user_participations);
}