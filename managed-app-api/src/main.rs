use async_graphql::{
    Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::post, Router};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;

type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject, FromRow)]
struct ManagedApp {
    id: i64,
    name: String,
    endpoint: Option<String>,
    version: Option<String>,
}

#[derive(SimpleObject)]
struct ServiceStatus {
    state: String,
    message: Option<String>,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn managed_apps(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<ManagedApp>> {
        let pool = ctx.data::<PgPool>()?;

        let apps = sqlx::query_as::<_, ManagedApp>(
            r#"
            SELECT
                a.id,
                a.name,
                (
                    SELECT e.url
                    FROM endpoint e
                    WHERE e.app_id = a.id
                    ORDER BY e.id
                    LIMIT 1
                ) AS endpoint,
                v.version
            FROM application a
            LEFT JOIN app_version v ON v.app_id = a.id
            ORDER BY a.name
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(apps)
    }

    // Placeholder until you connect a real health check.
    async fn podman_repo_status(&self) -> ServiceStatus {
        ServiceStatus {
            state: "UNKNOWN".into(),
            message: Some("Health check is not configured".into()),
        }
    }

    // Placeholder until you connect a real build-worker status check.
    async fn build_server_status(&self) -> ServiceStatus {
        ServiceStatus {
            state: "UNKNOWN".into(),
            message: Some("Build-server status check is not configured".into()),
        }
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn change_app_version(
        &self,
        ctx: &Context<'_>,
        app_id: i64,
        version: String,
    ) -> async_graphql::Result<ManagedApp> {
        let pool = ctx.data::<PgPool>()?;

        let version = version.trim();
        if version.is_empty() {
            return Err("Version must not be empty".into());
        }

        let app = sqlx::query_as::<_, ManagedApp>(
            r#"
            WITH updated AS (
                INSERT INTO app_version (app_id, version)
                VALUES ($1, $2)
                ON CONFLICT (app_id)
                DO UPDATE SET version = EXCLUDED.version,
                              updated_at = now()
            )
            SELECT
                a.id,
                a.name,
                (
                    SELECT e.url
                    FROM endpoint e
                    WHERE e.app_id = a.id
                    ORDER BY e.id
                    LIMIT 1
                ) AS endpoint,
                v.version
            FROM application a
            LEFT JOIN app_version v ON v.app_id = a.id
            WHERE a.id = $1
            "#
        )
        .bind(app_id)
        .bind(version)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("Application not found"))?;

        Ok(app)
    }

    // These return errors rather than pretending an action happened.
    async fn restart_podman_repo(&self) -> async_graphql::Result<String> {
        Err("Restart action is not configured".into())
    }

    async fn run_build(&self) -> async_graphql::Result<String> {
        Err("Build action is not configured".into())
    }
}

async fn graphql_handler(
    axum::extract::State(schema): axum::extract::State<ApiSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

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
