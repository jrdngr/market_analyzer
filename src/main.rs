pub mod analysis;
pub mod data_apis;
pub mod db;
pub mod graphql;
pub mod math;
pub mod types;
pub mod utils;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use warp::{
    http::{Response, StatusCode},
    Filter, Rejection,
};

use crate::{
    analysis::gamma_exposure::{gamma_exposure, gamma_exposure_aggregate},
    db::FileDb,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let frontend = warp::fs::dir("frontend/public");

    let db = match FileDb::load() {
        Ok(db) => db,
        Err(_) => FileDb::new(),
    };
    let db = Arc::new(Mutex::new(db));

    start_db_update_loop(db.clone())?;

    let graphql_filter = warp::path("graphql").and(
        async_graphql_warp::graphql(graphql::schema(db.clone())).and_then(
            |(schema, request): (graphql::Schema, async_graphql::Request)| async move {
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
            },
        ),
    );

    let graphql_playground = warp::path("playground").and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
        .allow_header("content-type");

    let routes = frontend.or(graphql_playground).or(graphql_filter);

    // If using something like `pretty_env_logger`,
    // view logs by setting `RUST_LOG=example::api`.
    let log = warp::log("ma::api");

    warp::serve(routes.recover(handle_rejection).with(log).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status(
        format!("{:?}", err),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

fn start_db_update_loop(db: Arc<Mutex<FileDb>>) -> anyhow::Result<()> {
    tokio::task::spawn(async move {
        let mut option_chain_interval = tokio::time::interval(Duration::from_secs(60 * 60));
        loop {
            option_chain_interval.tick().await;
            if let Err(e) = update_data(db.clone()).await {
                log::error!("{}", e);
            }
        }
    });

    Ok(())
}

async fn update_data(db: Arc<Mutex<FileDb>>) -> anyhow::Result<()> {
    use data_apis::tradier;

    let mut symbol_delay = tokio::time::interval(Duration::from_secs(30));
    let mut db = db.lock().await;

    for symbol in db.symbols() {
        log::info!("Updating data for {}", symbol);
        let option_chain = tradier::get_option_chain(&symbol.to_uppercase()).await?;
        for option in &option_chain {
            db.add_option_info(option.clone())?;
        }

        let gex = gamma_exposure(&symbol, &option_chain)?;
        db.add_gamma_exposure(gex)?;

        let gex_agg = gamma_exposure_aggregate(&symbol, &option_chain)?;
        db.add_gamma_exposure_aggregate(gex_agg)?;

        symbol_delay.tick().await;
    }

    Ok(())
}
