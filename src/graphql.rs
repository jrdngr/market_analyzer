use std::sync::Arc;

use crate::{
    data_apis::tradier,
    db::FileDb,
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
        let quote = tradier::get_quote(&symbol).await?;
        Ok(quote.into())
    }

    async fn ohlc(
        &self,
        symbol: String,
        #[graphql(default_with = "default_interval()")] interval: OhlcInterval,
    ) -> anyhow::Result<Vec<Ohlc>> {
        let ohlc = tradier::get_time_and_sales(&symbol, interval).await?;
        let result = ohlc.into_iter().map(|ts| (interval, ts).into()).collect();
        Ok(result)
    }

    async fn gamma_exposure(
        &self,
        context: &Context<'_>,
        symbol: String,
    ) -> anyhow::Result<GammaExposureStats> {
        let db = context
            .data::<FileDb>()
            .map_err(|_| anyhow::anyhow!("Failed to load db"))?;
        let gex = db
            .current_gamma_exposure(&symbol)
            .ok_or_else(|| anyhow::anyhow!("No data found"))?;
        Ok(gex.clone())
    }

    async fn gamma_exposure_aggregate(
        &self,
        context: &Context<'_>,
        symbol: String,
    ) -> anyhow::Result<GammaExposureStats> {
        let db = context
            .data::<FileDb>()
            .map_err(|_| anyhow::anyhow!("Failed to load db"))?;
        let gex = db
            .current_gamma_exposure_aggregate(&symbol)
            .ok_or_else(|| anyhow::anyhow!("No data found"))?;
        Ok(gex.clone())
    }
}

fn default_interval() -> OhlcInterval {
    OhlcInterval::FiveMinute
}
