const USERNAME_MAX_LENGTH: usize = 20;
const PASSWORD_MAX_LENGTH: usize = 100;

#[derive(Debug, async_graphql::InputObject)]
pub struct NewUser {
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}

// TODO: Create something to auto create these fields based on the OG struct.
// All fields will have the same name as the OG struct, but just be an
// `Option<String>`.
#[derive(async_graphql::SimpleObject)]
pub struct NewUserError {
    username: Option<String>,
    password: Option<String>,
}

impl NewUser {
    pub fn parse(&self) -> Result<NewUser, NewUserError> {
        let mut error = NewUserError {
            username: None,
            password: None,
        };

        if self.username.len() <= 0 || self.username.len() > USERNAME_MAX_LENGTH {
            error.username = Some(format!(
                "Username length must be between 1 and {} characters long",
                USERNAME_MAX_LENGTH
            ))
        }

        if self.password.len() <= 0 || self.password.len() > PASSWORD_MAX_LENGTH {
            error.password = Some(format!(
                "Password must be between 1 and {} characters long",
                PASSWORD_MAX_LENGTH
            ))
        }

        if error.username.is_none() && error.password.is_none() {
            Ok(NewUser {
                username: self.username.to_string(),
                password: self.password.to_string(),
            })
        } else {
            Err(error)
        }
    }
}
