use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::Claims;
use crate::user::models::User;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

// TODO: Config
const SECRET: &[u8] = b"iamsecret";

pub fn create_token(user: &User, auth_duration_in_hours: u16) -> ServiceResult<String> {
    let claims = Claims::new(user, auth_duration_in_hours);

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> ServiceResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
