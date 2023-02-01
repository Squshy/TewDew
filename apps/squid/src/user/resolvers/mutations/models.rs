use crate::jwt::models::Token;
use crate::user::models::User;

#[derive(async_graphql::SimpleObject)]
pub struct Memer {
    pub user: User,
    pub token: Token,
}
