use std::sync::Arc;

use crate::{
    analysis::gamma_exposure::{gamma_exposure, gamma_exposure_aggregate},
    data_apis::tradier,
    db::{self, FileDb},
    types::{GammaExposureStats, Ohlc, OhlcInterval, Quote},
};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object};
use tokio::sync::Mutex;

pub type Schema = async_graphql::Schema<Root, EmptyMutation, EmptySubscription>;

pub fn schema(db: Arc<Mutex<FileDb>>) -> Schema {
    async_graphql::Schema::build(Root, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}

pub struct Root;

#[Object]
impl Root {
    async fn quote(&self, symbol: String) -> anyhow::Result<Quote> {
        log::info!("Querying quote");
        let quote = tradier::get_quote(&symbol).await.map_err(log_error)?;
        Ok(quote.into())
    }

    async fn ohlc(
        &self,
        symbol: String,
        #[graphql(default_with = "default_interval()")] interval: OhlcInterval,
    ) -> anyhow::Result<Vec<Ohlc>> {
        log::info!("Querying ohlc");
        let ohlc = tradier::get_time_and_sales(&symbol, interval)
            .await
            .map_err(log_error)?;
        let result = ohlc.into_iter().map(|ts| (interval, ts).into()).collect();
        Ok(result)
    }

    async fn gamma_exposure(
        &self,
        context: &Context<'_>,
        symbol: String,
    ) -> anyhow::Result<GammaExposureStats> {
        log::info!("Querying gamma exposure");
        let db = context
            .data::<Arc<Mutex<FileDb>>>()
            .map_err(|_| anyhow::anyhow!("Failed to load db"))?;
        let option_chain = db::option_chain(&symbol, db.clone())
            .await
            .map_err(log_error)?;
        let gex = gamma_exposure(&symbol, &option_chain).unwrap();
        Ok(gex)
    }

    async fn gamma_exposure_aggregate(
        &self,
        context: &Context<'_>,
        symbol: String,
    ) -> anyhow::Result<GammaExposureStats> {
        log::info!("Querying gamma exposure aggregate");
        let db = context
            .data::<Arc<Mutex<FileDb>>>()
            .map_err(|_| anyhow::anyhow!("Failed to load db"))?;
        let option_chain = db::option_chain(&symbol, db.clone())
            .await
            .map_err(log_error)?;
        let gex_agg = gamma_exposure_aggregate(&symbol, &option_chain).unwrap();
        Ok(gex_agg)
    }
}

fn default_interval() -> OhlcInterval {
    OhlcInterval::FiveMinute
}

fn log_error(error: anyhow::Error) -> anyhow::Error {
    log::error!("{}", error);
    error
}
