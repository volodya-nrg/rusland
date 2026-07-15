mod configs;
mod logger;

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use std::error::Error;
// use sqlx::Connection;
use sqlx::FromRow;
use tokio::net::TcpListener;
use tower_http::services::fs::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    if let Err(e) = run("./config.yaml").await {
        log::error!("failed to run app: {e}");
        std::process::exit(1);
    }
}

async fn run(config_filepath: &str) -> Result<(), Box<dyn Error>> {
    let cfg = configs::new(config_filepath)?;

    logger::init(cfg.service_name, cfg.version, cfg.log.level);

    let listener: TcpListener = TcpListener::bind(&cfg.http_server.address).await?;
    let router: Router = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../web/index.html")) }),
        )
        .fallback_service(
            ServeDir::new("./web")
                // предварительно сжатые файлы
                .precompressed_gzip()
                .precompressed_br()
                .not_found_service(ServeFile::new("./web/404.html")),
        );
    // let conn = Connection::connect("sqlite:./data/db.sqlite3").await.unwrap();

    log::info!("start server on {}", &cfg.http_server.address);
    axum::serve(listener, router).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct State {
    // db: SqliteConnection,
}
impl State {}

#[derive(FromRow)]
struct OrderRecord {
    order_id: String,
    fio: String,
    tel: String,
    email: String,
    description: String,
    created_at: String,
}
