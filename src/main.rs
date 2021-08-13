pub mod analysis;
pub mod data_apis;
pub mod db;
pub mod graphql;
pub mod math;
pub mod types;
pub mod utils;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use tokio::sync::Mutex;
use std::{convert::{Infallible, TryInto}, sync::Arc, time::Duration};
use warp::{
    http::{Response, StatusCode},
    Filter, Rejection,
};

use crate::db::FileDb;

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

    let graphql_filter = warp::path("graphql").and(async_graphql_warp::graphql(db.clone())).and_then(
        |(schema, request): (graphql::Schema, async_graphql::Request)| async move {
            let resp = schema.execute(request).await;
            Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
        },
    ));

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
    use data_apis::tradier;

    tokio::task::spawn(async move {
        let mut option_chain_interval = tokio::time::interval(Duration::from_secs(60 * 60));
        let mut symbol_delay = tokio::time::interval(Duration::from_secs(30));

        loop {
            option_chain_interval.tick().await;
            let db = db.lock().await;
            for symbol in db.symbols() {
                match tradier::get_option_chain(&symbol.to_uppercase()).await {
                    Ok(option_chain) => {
                        for option in option_chain {
                            match option.try_into() {
                                Ok(o) => if let Err(e) = db.add_option_info(o) {
                                    log::error!("{}", e);
                                }
                                Err(e) => log::error!("{}", e),
                            }
                        }

                        let gex = analysis::gamma_exposure()
                    }
                    Err(e) => {
                        log::error!("{}", e)
                    }
                }
            }
        }
    });

    Ok(())
}
