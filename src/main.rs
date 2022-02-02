pub mod analysis;
pub mod data_apis;
pub mod db;
pub mod graphql;
pub mod math;
pub mod types;
pub mod utils;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
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
    let db_download = warp::path("db").and(warp::fs::file("data/db.gz"));

    let db = match FileDb::load() {
        Ok(db) => db,
        Err(_) => FileDb::default(),
    };
    let db = Arc::new(Mutex::new(db));

    db::start_db_update_loop(db.clone())?;

    let tradier_graphql_filter = warp::path("graphql").and(
        async_graphql_warp::graphql(graphql::schema(db.clone())).and_then(
            |(schema, request): (graphql::Schema, async_graphql::Request)| async move {
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
            },
        ),
    );

    let tradier_graphql_playground = warp::path("playground").and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let tda_graphql_filter = warp::path("tdagraphql").and(
        async_graphql_warp::graphql(graphql::tda_schema(db.clone())).and_then(
            |(schema, request): (graphql::TdaSchema, async_graphql::Request)| async move {
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(async_graphql_warp::Response::from(resp))
            },
        ),
    );

    let tda_graphql_playground = warp::path("tdaplayground").and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new(
                "/tdagraphql",
            )))
    });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
        .allow_header("content-type");

    let routes = db_download
        .or(tradier_graphql_filter)
        .or(tradier_graphql_playground)
        .or(tda_graphql_filter)
        .or(tda_graphql_playground)
        .or(frontend);

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
