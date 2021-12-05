use async_graphql::{ComplexObject, SimpleObject};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct RowId {
    pub id: Uuid
}

#[derive(Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[ComplexObject]
impl User {}
