use async_graphql::SimpleObject;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct RowId {
    pub id: Uuid,
}

#[derive(Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, FromRow, SimpleObject)]
pub struct Person {
    pub id: Uuid,
    pub name: Vec<u8>,
    pub user_id: Uuid,
}
