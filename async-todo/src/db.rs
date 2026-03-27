pub async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());
    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("cannot connect to database");

    tracing::info!("running database migrations");

    sqlx::migrate!("./migrations")
        .run(&dbpool)
        .await
        .expect("database migration failed");

    tracing::info!("database migrations complete");

    Ok(dbpool)
}
