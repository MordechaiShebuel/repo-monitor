mod graphql;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

async fn graphql_handler(
    State(schema): State<graphql::ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let schema = graphql::build_schema(pool);

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .with_state(schema);

    let addr: SocketAddr = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8000".into())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("GraphQL API listening at http://{addr}/graphql");

    axum::serve(listener, app).await?;
    Ok(())
}
