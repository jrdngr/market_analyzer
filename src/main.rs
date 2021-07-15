pub mod gamma_exposure;
pub mod td;
pub mod utils;

use std::convert::Infallible;

use warp::{Filter, Rejection, http::StatusCode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let gamma_exposure = warp::get()
        .and(warp::path!("gamma" / String))
        .and_then(gamma_exposure);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT"]);

    let routes = gamma_exposure;

    warp::serve(routes.recover(handle_rejection).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn gamma_exposure(symbol: String) -> Result<impl warp::Reply, Rejection> {
    match gamma_exposure::gamma_exposure_by_price(&symbol).await {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| warp::reject::not_found())?),
        Err(_) => Err(warp::reject::not_found()), 
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("{:?}", err);
    Ok(warp::reply::with_status(
        format!("{:?}", err),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
