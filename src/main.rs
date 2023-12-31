pub mod db;
pub mod handlers;
pub mod service;
pub mod models;
pub mod dto;
pub mod errors;

use actix_web::{HttpServer, App, web};
use db::init_db_pool;
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
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/events")
                    .wrap(service::auth::AuthMiddleware{
                        db_pool: pool.clone()
                    })
            )
            .service(
                web::scope("/users")
                .configure(handlers::user::config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
