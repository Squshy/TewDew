use unicode_segmentation::UnicodeSegmentation;
const USERNAME_MAX_LENGTH: usize = 20;
const PASSWORD_MAX_LENGTH: usize = 100;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct NewUser {
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}

// TODO: Create something to auto create these fields based on the OG struct.
// All fields will have the same name as the OG struct, but just be an
// `Option<String>`.
#[derive(Debug, async_graphql::SimpleObject)]
pub struct NewUserError {
    username: Option<String>,
    password: Option<String>,
}

impl NewUser {
    pub fn new(username: String, password: String) -> Result<NewUser, NewUserError> {
        let mut error = NewUserError {
            username: None,
            password: None,
        };

        match Self::check_length(&username, USERNAME_MAX_LENGTH) {
            Ok(_) => (),
            Err(e) => error.username = Some(format!("Username {}", e)),
        }

        match Self::check_length(&password, PASSWORD_MAX_LENGTH) {
            Ok(_) => (),
            Err(e) => error.password = Some(format!("Password {}", e)),
        }

        if error.username.is_none() && error.password.is_none() {
            Ok(NewUser { username, password })
        } else {
            Err(error)
        }
    }

    fn check_length(value: &String, max_length: usize) -> Result<(), String> {
        if value.trim().is_empty() {
            return Err("must not be empty".into());
        }

        // Graphemes are perceived as a single character but it is combined of
        // two characters. For example a french letter with an accent + an english
        // letter
        if value.graphemes(true).count() > max_length {
            return Err(format!("must be less than {} characters long", max_length));
        }

        // Forbidden characters?

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{NewUser, PASSWORD_MAX_LENGTH, USERNAME_MAX_LENGTH};
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_max_length_grapheme_username_and_password_is_accepted() {
        let username = "ё".repeat(USERNAME_MAX_LENGTH);
        let password = "ё".repeat(PASSWORD_MAX_LENGTH);

        assert_ok!(NewUser::new(username, password));
    }

    #[test]
    fn a_blank_username_is_rejected() {
        let username = "".to_string();
        let password = "a".to_string();

        assert_err!(NewUser::new(username, password));
    }

    #[test]
    fn a_blank_password_is_rejected() {
        let username = "a".to_string();
        let password = "".to_string();

        assert_err!(NewUser::new(username, password));
    }

    #[test]
    fn username_over_character_limit_is_rejected() {
        let username = "m".repeat(USERNAME_MAX_LENGTH + 1);
        let password = "abc".to_string();

        assert_err!(NewUser::new(username, password));
    }

    #[test]
    fn password_over_character_limit_is_rejected() {
        let username = "m".repeat(USERNAME_MAX_LENGTH);
        let password = "m".repeat(PASSWORD_MAX_LENGTH + 1);

        assert_err!(NewUser::new(username, password));
    }
}
