use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(SimpleObject, FromRow)]
pub struct ManagedApp {
    pub id: i64,
    pub name: String,
    pub endpoint: Option<String>,
    pub version: Option<String>,
}

#[derive(SimpleObject)]
pub struct ServiceStatus {
    pub state: String,
    pub message: Option<String>,
}
