use async_graphql::{Context, Object};
use sqlx::PgPool;

use super::types::ManagedApp;

pub(crate) struct MutationRoot;

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
            "#,
        )
        .bind(app_id)
        .bind(version)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("Application not found"))?;

        Ok(app)
    }

    async fn restart_podman_repo(&self) -> async_graphql::Result<String> {
        Err("Restart action is not configured".into())
    }

    async fn run_build(&self) -> async_graphql::Result<String> {
        Err("Build action is not configured".into())
    }
}
