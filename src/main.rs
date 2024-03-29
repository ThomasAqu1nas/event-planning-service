pub mod db;
pub mod handlers;
pub mod service;
pub mod models;
pub mod dto;
pub mod errors;

use actix_web::{HttpServer, App, web, HttpResponse};
use db::init_db_pool;
use dto::Routes;
use service::{auth::AuthMiddleware, log::LoggerMiddleware};
use sqlx::{postgres::Postgres, Pool};
use dotenv::dotenv;
use std::env;

type PGPool = Pool<Postgres>;

const ACCESS_TOKEN_EXP: usize = 60 * 60 * 1000 * 1000;
const REFRESH_TOKEN_EXP: usize = 5 * 24 * 60 * 60 * 1000 * 1000;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
    .unwrap_or_else(|e| {
        panic!("Failed to get env with name 'DATABASE_URL': {:?}", e);
    });
    let pool: PGPool = init_db_pool(&db_url).await;

    let info = || async {
        let routes = Routes { 
            auth: vec!["/login".to_string(), "register".to_string()], 
            event: vec![
                "/create".to_string(), 
                "/{id}/subscribe".to_string(),
                "/".to_string(),
                "/{id}".to_string(),
                "/update/{id}".to_string(),
                "/{id}/invitaion".to_string(),
                "/{id}/accept-invitation".to_string()
            ], 
            user: vec![
                "/".to_string(),
                "/{id}".to_string(),
                "/{id}/participations".to_string()
            ]
        };
        
        HttpResponse::Ok().json(routes)
    };
    service::log::init_logger();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(info))
            .service(
                web::scope("/user")
                    .wrap(LoggerMiddleware) 
                    .configure(handlers::user::init_routes)  
            )
            .service(
                web::scope("/event")
                    .wrap(AuthMiddleware::register(pool.clone()))
                    .wrap(LoggerMiddleware)
                    .configure(handlers::event::init_routes)
            )
            .service(
                web::scope("/auth")
                .wrap(LoggerMiddleware)
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("register", web::post().to(handlers::auth::register))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
