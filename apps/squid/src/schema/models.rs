use async_graphql::InputObject;

#[derive(InputObject)]
pub struct ListParams {
    pub skip: i16,
    pub limit: i16,
}
