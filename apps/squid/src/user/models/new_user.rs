use crate::schema::models::FieldError;
use crate::validation::check_length;
const USERNAME_MAX_LENGTH: usize = 20;
const PASSWORD_MAX_LENGTH: usize = 100;

#[derive(Debug, Clone)]
pub struct NewUser {
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}

impl NewUser {
    pub fn new(username: String, password: String) -> Self {
        NewUser { username, password }
    }

    pub fn errors(&self) -> Option<Vec<FieldError>> {
        let mut user_errors: Vec<FieldError> = vec![];

        if let Err(err) = check_length(&self.username, USERNAME_MAX_LENGTH) {
            user_errors.push(FieldError {
                field: "username".into(),
                message: err,
            });
        }

        if let Err(err) = check_length(&self.password, PASSWORD_MAX_LENGTH) {
            user_errors.push(FieldError {
                field: "password".into(),
                message: err,
            });
        }

        if user_errors.is_empty() {
            return None;
        }

        Some(user_errors)
    }
}
