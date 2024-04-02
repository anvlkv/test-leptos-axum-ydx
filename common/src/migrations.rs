use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!().run(pool).await.unwrap();
}
