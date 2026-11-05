use async_graphql::{Context, Object};
use sqlx::PgPool;

use super::types::{ManagedApp, ServiceStatus};

pub(crate) struct QueryRoot;

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
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(apps)
    }

    async fn podman_repo_status(&self) -> ServiceStatus {
        ServiceStatus {
            state: "UNKNOWN".into(),
            message: Some("Health check is not configured".into()),
        }
    }

    async fn build_server_status(&self) -> ServiceStatus {
        ServiceStatus {
            state: "UNKNOWN".into(),
            message: Some("Build-server status check is not configured".into()),
        }
    }
}
