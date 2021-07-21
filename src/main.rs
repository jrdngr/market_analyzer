pub mod data_apis;
pub mod gamma_exposure;
pub mod utils;

use data_apis::tradier;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let gamma_exposure = warp::get()
        .and(warp::path!("gamma" / String))
        .and_then(handle_gamma_exposure);

    let quote = warp::get()
        .and(warp::path!("quote" / String))
        .and_then(handle_quote);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT"]);

    let routes = gamma_exposure.or(quote);

    warp::serve(routes.recover(handle_rejection).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

// async fn gamma_exposure(symbol: String) -> Result<impl warp::Reply, Rejection> {
//     handle_gamma_exposure(symbol).await
// }

async fn handle_gamma_exposure(symbol: String) -> Result<impl warp::Reply, Rejection> {
    match gamma_exposure::gamma_exposure_by_price(&symbol).await {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| warp::reject::not_found())?),
        Err(err) => {
            log::error!("{:?}", err);
            Err(warp::reject::not_found())
        }
    }
}

// async fn quote(symbol: String) -> Result<impl warp::Reply, Rejection> {
//     handle_quote(symbol).await
// }

async fn handle_quote(symbol: String) -> Result<impl warp::Reply, Rejection> {
    match tradier::get_quote(&symbol).await {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| warp::reject::not_found())?),
        Err(err) => {
            log::error!("{:?}", err);
            Err(warp::reject::not_found())
        }
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status(
        format!("{:?}", err),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
