use actix_web::{Responder, web, post, HttpResponse};

use crate::{PGPool, dto::{LoginUserRequest, NewUserDto}, service};

#[post("/login")]
pub async fn login(dto: web::Json<LoginUserRequest>, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    println!("{:?}", "REG");
    let response = service::auth::jwt::login(conn, dto.0).await;
    match response {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => HttpResponse::InternalServerError().json(err)
    }
}

#[post("/register")]
pub async fn register(dto: web::Json<NewUserDto>, pool_state: web::Data<PGPool>) -> impl Responder {
    let conn: &PGPool = pool_state.get_ref();
    println!("{:?}", "REG");
    let response = service::user::create(dto.0, conn).await;
    match response {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create new user")
    }
}