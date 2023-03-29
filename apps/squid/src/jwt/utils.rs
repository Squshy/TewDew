use crate::errors::ServiceError;
use crate::errors::ServiceResult;
use crate::jwt::errors::JWTError;
use crate::jwt::models::Claims;
use crate::jwt::models::Token;
use crate::user::models::User;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

// TODO: Config
const SECRET: &[u8] = b"iamsecret";

pub fn create_token(user: &User, auth_duration_in_hours: u16) -> ServiceResult<Token> {
    let claims = Claims::new(user, auth_duration_in_hours);

    let bearer = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(Token { bearer })
}

pub fn decode_token(token: &str) -> ServiceResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|_| JWTError::InvalidToken.into())
}
