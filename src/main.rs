pub mod analysis;
pub mod data_apis;
pub mod math;
pub mod utils;

use axum::prelude::*;
use data_apis::tradier;
use hyper::StatusCode;
use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let app = route("/ohlc", get(handle_ohlc))
        .route("/gamma", get(handle_gamma_exposure))
        .route("/quote", get(handle_quote));

    hyper::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct GammaExposureOptions {
    aggregate: bool,
    fresh: bool,
}

async fn handle_ohlc(
    extract::UrlParams(params): extract::UrlParams<(String, String)>,
) -> Result<String, StatusCode> {
    match tradier::get_time_and_sales(&params.0, &params.1).await {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        Err(err) => {
            log::error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn handle_gamma_exposure(
    extract::UrlParams(params): extract::UrlParams<(String,)>,
    options: extract::Query<GammaExposureOptions>,
) -> Result<String, StatusCode> {
    let gamma_exposure = if options.aggregate {
        analysis::gamma_exposure::gamma_exposure_aggregate(&params.0, options.fresh).await
    } else {
        analysis::gamma_exposure::gamma_exposure_stats(&params.0, options.fresh).await
    };

    match gamma_exposure {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        Err(err) => {
            log::error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn handle_quote(
    extract::UrlParams(params): extract::UrlParams<(String,)>,
) -> Result<String, StatusCode> {
    match tradier::get_quote(&params.0).await {
        Ok(ge) => Ok(serde_json::to_string(&ge).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        Err(err) => {
            log::error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
