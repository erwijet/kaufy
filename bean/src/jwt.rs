use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use poem_grants::error::AccessError::UnauthorizedRequest;
use serde::{Deserialize, Serialize};

const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub id: i64,
    exp: i64,
}

impl From<entity::user::Model> for Claims {
    fn from(value: entity::user::Model) -> Self {
        Self {
            email: value.email,
            family_name: value.family_name,
            given_name: value.given_name,
            id: value.id as i64,
            picture: value.picture,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(claims: Claims) -> poem::Result<String> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| UnauthorizedRequest.into()) // Just example, here should be the correct way to handle the error
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> poem::Result<Claims> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| UnauthorizedRequest.into()) // Just example, here should be the correct way to handle the error
}
