use crate::{dto::NewUserDto, PGPool, models::{User, Event}, errors::MyError, service::auth, ACCESS_TOKEN_EXP};
use crate::db;
use uuid::Uuid;

use super::crypto;

pub async fn create(dto: NewUserDto, pool: &PGPool) -> Result<u64, MyError>{
    let NewUserDto{username, email, pwd, pwd_confirm} = dto;
    if db::user::exists(username.clone(), pool).await {
        return Err(MyError::BadClientData);
    } else {
        let pwd_hash: String = crypto::get_sha3_256_hash(&pwd);
        let pwd_confirm_hash: String = crypto::get_sha3_256_hash(&pwd_confirm);
        let access_token: Option<String>;
        let refresh_token = None::<String>;
        let id = Uuid::new_v4();
        
        if let Ok(v) = auth::jwt::create(
            &auth::jwt::TokenType::Access, &id, &username, ACCESS_TOKEN_EXP
        ) {
            access_token = Some(v);
        } else {
            return Err(MyError::AuthError);
        }
        if pwd_hash.eq(&pwd_confirm_hash) {
            let res = db::user::create(User { 
                id,
                pwd_hash, 
                username, 
                email,
                access_token,
                refresh_token
            }, pool)
            .await;
            dbg!(&res);
            match res {
                Ok(value) => Ok(value.rows_affected()),
                Err(_) => Err(MyError::BadClientData)
            }
        } else {
            return Err(MyError::BadClientData);
        }
    }
}

pub async fn get_all(pool: &PGPool) -> Result<Vec<User>, MyError> {
    let result = db::user::get_all(pool).await;
    match result {
        Ok(users) => Ok(users),
        Err(_) => Err(MyError::InternalError)
    }
}

pub async fn get_by_id(id: Uuid, pool: &PGPool) -> Result<User, MyError> {
    let result = db::user::get_by_id(id, pool)
        .await;
    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(MyError::InternalError)
    }
}

pub async fn get_user_participations(id: Uuid, pool: &PGPool) -> Result<Vec<Event>, MyError> {
    let result = db::user::get_user_participations(id, pool)
        .await;
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(MyError::InternalError)
    }
}

