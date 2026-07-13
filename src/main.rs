mod configs;
mod logger;

use axum::Router;
use axum::routing::get;
use std::error::Error;
// use sqlx::Connection;
use sqlx::FromRow;
use tokio::net::TcpListener;

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
    let router: Router = Router::new().route("/", get(handler_main).with_state(State {}));
    // let conn = Connection::connect("sqlite:./data/db.sqlite3").await.unwrap();

    log::info!("start server on {}", &cfg.http_server.address);
    axum::serve(listener, router).await?;
    Ok(())
}

async fn handler_main() -> &'static str {
    "Hello, world!"
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
