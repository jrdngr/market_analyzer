pub mod gamma_exposure;
pub mod td;
pub mod utils;

use std::convert::Infallible;

use warp::{http::StatusCode, Filter, Rejection};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let gamma_exposure = warp::get()
        .and(warp::path!("gamma" / String))
        .and_then(gamma_exposure);

    let gamma_exposure_fresh = warp::get()
        .and(warp::path!("gamma" / String / "fresh"))
        .and_then(gamma_exposure_fresh);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT"]);

    let routes = gamma_exposure.or(gamma_exposure_fresh);

    warp::serve(routes.recover(handle_rejection).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn gamma_exposure(symbol: String) -> Result<impl warp::Reply, Rejection> {
    handle_gamma_exposure(symbol, false).await
}

async fn gamma_exposure_fresh(symbol: String) -> Result<impl warp::Reply, Rejection> {
    handle_gamma_exposure(symbol, true).await
}

async fn handle_gamma_exposure(
    symbol: String,
    force_download: bool,
) -> Result<impl warp::Reply, Rejection> {
    match gamma_exposure::gamma_exposure_by_price(&symbol, force_download).await {
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
