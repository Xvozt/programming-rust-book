use async_todo::create_router;
use async_todo::init_dbpool;

#[tokio::main]
async fn main() {
    init_tracing();
    let dbpool = init_dbpool().await.expect("couldn't initialize db pool");

    let router = create_router(dbpool).await;
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .expect("Failed to bind port");

    axum::serve(listener, router.into_make_service())
        .await
        .expect("unable to start service");
}

fn init_tracing() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}
