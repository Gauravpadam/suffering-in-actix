

use std::future;

use actix_web::{FromRequest, HttpMessage};
use chrono::{Utc, Duration};
use super::constants;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};


#[derive(Serialize, Deserialize, Clone)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32
}

impl FromRequest for Claims{
    type Error = actix_web::Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> future::Ready<Result<Claims, actix_web::Error>>{

        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Request")))
        }

    }
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error>{

    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims{
        exp: (now+expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id
    };

    let secret = (*constants::SECRET).clone();

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))

}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>,jsonwebtoken::errors::Error> {
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    claim_data
}