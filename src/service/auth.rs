use std::future::{ready, Ready};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, http::header::HeaderValue, HttpMessage};
use futures_util::future::LocalBoxFuture;
use crate::{PGPool, ACCESS_TOKEN_EXP, db, dto::UpdateUserDto};

use self::jwt::TokenType;

pub struct UserAuthData{
    pub user_id: uuid::Uuid,
    pub username: String
}

pub struct AuthMiddleware {
    pub db_pool: PGPool
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware 
    where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthMiddlewareSerive<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareSerive {
            service,
            db_pool: self.db_pool.clone() 
        }))
    }
}


pub struct AuthMiddlewareSerive<S> {
    service: S,
    db_pool: PGPool
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareSerive<S>
    where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let access_token_validation_result: Result<String, crate::errors::MyError> = jwt::validate(&req, TokenType::Access, "Bearer");
        let refresh_token_validation_result: Result<String, crate::errors::MyError> = jwt::validate(&req, TokenType::Refresh, "Bearer");
        let pool = self.db_pool.clone();
        match (access_token_validation_result, refresh_token_validation_result) {
            (Ok(_), Ok(refresh_token)) => {
                match jwt::decode_claims(&TokenType::Refresh, refresh_token) {
                    Ok(claims) => {
                        let user_auth_data = UserAuthData {
                            user_id: claims.claims.user_id,
                            username: claims.claims.username,
                        };
                        req.extensions_mut().insert(user_auth_data);
                    },
                    _ => {}
                }
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            },
            (Err(_), Ok(_)) => {
                match jwt::parse_request(&req, "Bearer") {
                    Ok(token) => {
                        if let Ok(claims) = jwt::decode_claims(&TokenType::Access, token) {
                            let user_id = claims.claims.user_id;
                            let username = claims.claims.username;
                            let new_token = jwt::create(&TokenType::Access, &user_id, &username, ACCESS_TOKEN_EXP).unwrap();
                            let user_fields = UpdateUserDto {
                                pwd_hash: None,
                                username: None,
                                email: None,
                                access_token: Some(new_token.clone()),
                                refresh_token: None,
                            };

                            req.headers_mut().insert(
                                actix_web::http::header::AUTHORIZATION,
                                HeaderValue::from_str(&new_token).unwrap()
                            );
                            let fut = self.service.call(req);
                            Box::pin(async move {
                                let fields_res = db::user::set_fields(user_id, user_fields, &pool)
                                .await;
                                match fields_res {
                                    Ok(_) => {
                                        Ok(fut.await?)
                                    },
                                    Err(_) => {
                                        Err(actix_web::error::ErrorInternalServerError(""))
                                    }
                                }    
                            })
                        } else {
                            Box::pin(async move {
                                Err(actix_web::error::ErrorInternalServerError("login again"))
                            })
                        }
                    }, 
                    Err(_) => {
                        Box::pin(async move {
                            Err(actix_web::error::ErrorBadRequest("invalid request"))
                        })                        
                    }
                }
            },
            (_, Err(_)) => {
                Box::pin(async move {
                    Err(actix_web::error::ErrorBadRequest("register"))
                })  
            },
        }
    }
}


pub mod jwt {
    use std::env::{self, VarError};
    use actix_web::dev::ServiceRequest;
    use chrono::Utc;
    use dotenv::dotenv;
    use jsonwebtoken::{Header, Algorithm, EncodingKey, encode, decode, errors::Error, DecodingKey, Validation, TokenData};
    use crate::{dto::{Claims, self, UpdateUserDto}, errors::MyError, PGPool, db, ACCESS_TOKEN_EXP, REFRESH_TOKEN_EXP};

    pub enum TokenType {
        Refresh,
        Access
    }

    pub fn get_secret(token_type: &TokenType) -> Result<String, VarError> {
        dotenv().ok();
        let env_key: String;
        match token_type {
            TokenType::Refresh => env_key = "JWT_REFRESH_TOKEN".to_string(),
            TokenType::Access => env_key ="JWT_ACCESS_TOKEN".to_string()
        }
        let secret = env::var(env_key);
        secret
    }

    pub fn decode_claims(token_type: &TokenType, token: String) -> Result<TokenData<Claims>, Error> {
        let secret = get_secret(token_type).expect("Jwt token secret must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::RS256);
        let claims = decode::<Claims>(&token, &decoding_key, &validation);
        claims
    }

    pub fn create(token_type: &TokenType, user_id: &uuid::Uuid, username: &String, exp: usize) -> Result<String, Error> {
        let exp_timestamp = Utc::now().timestamp_micros() as usize + exp;
        let secret = get_secret(token_type).expect("Jwt token secret must be set");
        let header: Header = Header::new(Algorithm::RS256);
        let claims: Claims = Claims::new(user_id, username, exp_timestamp);
        let key: EncodingKey = EncodingKey::from_secret(secret.as_ref());
        let token_res = encode(&header, &claims, &key);
        match token_res {
            Ok(token) => Ok(token),
            Err(err) => Err(err)
        }
    } 

    /// refreshes given **`token`**
    pub async fn refresh(token: String, token_type: & TokenType, pool: & PGPool, new_exp: usize) -> Result<String, MyError> {
        let claims_result: Result<TokenData<Claims>, Error> = decode_claims(&token_type, token);
        let user_id: uuid::Uuid;
        let username: String;
        let updated_user_fields: UpdateUserDto;

        return if let Ok(expired_token_claims) = claims_result {
            user_id = expired_token_claims.claims.user_id;
            username = expired_token_claims.claims.username;
            let new_token_result = create(&token_type, &user_id, &username, new_exp);
            if let Ok(new_token) = new_token_result {
                updated_user_fields = UpdateUserDto {
                    pwd_hash: None,
                    username: None,
                    email: None,
                    access_token: Some(new_token.clone()),
                    refresh_token: None,
                };
                let res = db::user::set_fields(user_id, updated_user_fields, pool)
                    .await;
                if let Ok(_) = res {
                    Ok(new_token)
                } else {
                    Err(MyError::InternalError)
                }
            } else {
                Err(MyError::InternalError)
            }
        } else {
            Err(MyError::DecodeError)
        }
    }

    ///refreshes **`refresh_token`** if it expires</br>
    ///otherwise creates a new refresh token
    pub async fn login(pool: &PGPool, req: dto::LoginUserRequest) -> Result<(), MyError> {
        let user_id_result = db::user::get_id_by_username(req.username.clone(), pool)
            .await;
        return match user_id_result {
            Ok(user_id) => {
                let new_access_token: String;
                let new_refresh_token: String;
                if let Ok(access_token) = create(&TokenType::Access, &user_id, &req.username, ACCESS_TOKEN_EXP) {
                    new_access_token = access_token;
                } else {
                    return Err(MyError::BadClientData);
                }
                if let Ok(refresh_token) = create(&TokenType::Refresh, &user_id, &req.username, REFRESH_TOKEN_EXP) {
                    new_refresh_token = refresh_token;
                } else {
                    return Err(MyError::BadClientData);
                }
                let updated_user_fields = UpdateUserDto {
                    pwd_hash: None,
                    username: None,
                    email: None,
                    access_token: Some(new_access_token),
                    refresh_token: Some(new_refresh_token),
                };
                let res = db::user::set_fields(user_id, updated_user_fields, pool).await;
                match res {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(_) => {
                        Err(MyError::InternalError)
                    },
                }
            },
            Err(_) => {
                Err(MyError::BadClientData)
            },
        }
    }

    /// check if JW token is expired or not </br>
    /// returns **`MyError::AuthError`** if token is expired </br>
    /// else returns **`token`** itself</br>
    /// throws an **`MyError::DecodeError`** if the jwt decoding fails
    pub fn validate(req: &ServiceRequest, token_type: TokenType, prefix: &str) -> Result<String, MyError> {
        if let Ok(token) = parse_request(req, prefix) {
            let claims: Result<TokenData<Claims>, Error> = decode_claims(&token_type, format!("{token}"));
            return match claims {
                Ok(c) => {
                    if (Utc::now().timestamp_micros() as usize) > c.claims.exp {
                        Ok(token)
                    } else {
                        Err(MyError::AuthError)
                    }
                },
                Err(_) => {
                    Err(MyError::DecodeError)
                }
            }
        }
        Err(MyError::AuthError)
    }

    pub fn parse_request(req: &ServiceRequest, prefix: & str) -> Result<String, MyError> {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_value) = auth_header.to_str() {
                if let Some(token) = auth_value.strip_prefix(prefix) {
                    return Ok(token.to_string());
                }
            }
        }
        Err(MyError::AuthError)
    }
}