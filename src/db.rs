use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Application-wide shared state holding the database pool.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost:5432/codeflow".to_string());

        let db = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&database_url)
            .await?;

        // Run migrations at startup (fail-fast)
        sqlx::migrate!("./migrations").run(&db).await?;

        Ok(Self { db })
    }
}

pub async fn health_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Json<serde_json::Value> {
    let ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    axum::response::Json(serde_json::json!({
        "status": if ok { "healthy" } else { "degraded" },
        "service": "codeflow"
    }))
}
