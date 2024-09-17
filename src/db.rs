use sqlx::PgPool;

pub async fn setup_database(config: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(config).await
}