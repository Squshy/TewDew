use crate::user::models::User;
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user id)
    pub sub: Uuid,
    /// Issued at
    pub iat: i64,
    /// Expiry
    pub exp: i64,
}

impl Claims {
    pub fn new(user: &User, auth_duration_in_hours: u16) -> Self {
        let User { id, .. } = user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hours));

        Claims {
            sub: *id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Token {
    pub bearer: String,
}
