use crate::errors::{ServiceError, ServiceResult};
use crate::user::errors::UserError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn verify_password(
    password: &String,
    user_password: &String,
) -> Result<Option<bool>, UserError> {
    let parsed_hash = match PasswordHash::new(&user_password) {
        Ok(password_hash) => password_hash,
        Err(_) => return Err(UserError::InvalidUsernameOrPassword),
    };

    let password_bytes: Vec<u8> = password.clone().into_bytes();
    let is_oki_doki = Argon2::default()
        .verify_password(&password_bytes, &parsed_hash)
        .is_ok();

    if !is_oki_doki {
        return Err(UserError::InvalidUsernameOrPassword);
    }

    Ok(None)
}

pub fn hash_password(password: &String) -> ServiceResult<String> {
    let password_as_bytes: Vec<u8> = password.bytes().collect();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(&password_as_bytes, &salt)
        .map_err(|_| ServiceError::InternalServerError)?
        .to_string();

    Ok(password_hash)
}
