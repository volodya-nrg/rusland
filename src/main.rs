mod config;
mod logger;

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use clap::Parser;
use std::error::Error;
use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};
use tower_http::services::fs::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args.config.as_str()).await {
        log::error!("failed to run app: {e}");
        std::process::exit(1);
    }
}

async fn run(config_filepath: &str) -> Result<(), Box<dyn Error>> {
    let cfg = config::new(config_filepath).expect("failed to load config file");

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
                // .precompressed_gzip()
                // .precompressed_br()
                .not_found_service(ServeFile::new("./web/404.html")),
        );
    // let conn = Connection::connect("sqlite:./data/db.sqlite3").await.unwrap();

    log::info!("start server on {}", &cfg.http_server.address);
    axum::serve(listener, router).await?;
    Ok(())
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "./config.yaml")]
    config: String,
}
