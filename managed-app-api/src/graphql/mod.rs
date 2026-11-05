mod mutation;
mod query;
mod types;

use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

use mutation::MutationRoot;
use query::QueryRoot;

pub use types::{ManagedApp, ServiceStatus};

pub(crate) type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(pool: PgPool) -> ApiSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_schema() -> ApiSchema {
        let pool = PgPool::connect_lazy("postgres://test:test@localhost/test")
            .expect("valid test database URL");

        build_schema(pool)
    }

    #[tokio::test]
    async fn status_queries_return_unconfigured_status() {
        let response = test_schema()
            .execute(
                r#"
                {
                    podmanRepoStatus { state message }
                    buildServerStatus { state message }
                }
                "#,
            )
            .await;

        assert!(response.errors.is_empty(), "{:?}", response.errors);

        let data = response.data.into_json().expect("JSON response");
        assert_eq!(data["podmanRepoStatus"]["state"], "UNKNOWN");
        assert_eq!(
            data["podmanRepoStatus"]["message"],
            "Health check is not configured"
        );
        assert_eq!(data["buildServerStatus"]["state"], "UNKNOWN");
        assert_eq!(
            data["buildServerStatus"]["message"],
            "Build-server status check is not configured"
        );
    }

    #[tokio::test]
    async fn changing_app_version_rejects_blank_version() {
        let response = test_schema()
            .execute(
                r#"
                mutation {
                    changeAppVersion(appId: 1, version: "   ") {
                        id
                    }
                }
                "#,
            )
            .await;

        assert!(
            response
                .errors
                .iter()
                .any(|error| { error.message == "Version must not be empty" }),
            "Unexpected errors: {:?}",
            response.errors
        );
    }

    #[tokio::test]
    async fn restart_mutation_reports_unconfigured_action() {
        let response = test_schema()
            .execute(
                r#"
                mutation {
                    restartPodmanRepo
                }
                "#,
            )
            .await;

        assert!(
            response
                .errors
                .iter()
                .any(|error| { error.message == "Restart action is not configured" }),
            "Unexpected errors: {:?}",
            response.errors
        );
    }

    #[tokio::test]
    async fn build_mutation_reports_unconfigured_action() {
        let response = test_schema()
            .execute(
                r#"
                mutation {
                    runBuild
                }
                "#,
            )
            .await;

        assert!(
            response
                .errors
                .iter()
                .any(|error| { error.message == "Build action is not configured" }),
            "Unexpected errors: {:?}",
            response.errors
        );
    }
}
