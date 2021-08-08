use crate::{analysis, data_apis::tradier};
use async_graphql::{EmptyMutation, EmptySubscription, Object};

pub mod gex;
pub mod ohlc;
pub mod options;
pub mod quote;

pub use gex::{GammaExposure, GammaExposureOptions, GammaExposureStats};
pub use ohlc::{Ohlc, OhlcInterval};
pub use options::{OptionInfo, OptionType};
pub use quote::Quote;

pub type Schema = async_graphql::Schema<Root, EmptyMutation, EmptySubscription>;

pub fn schema() -> Schema {
    async_graphql::Schema::new(Root, EmptyMutation, EmptySubscription)
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
        symbol: String,
        #[graphql(default)] options: GammaExposureOptions,
    ) -> anyhow::Result<GammaExposureStats> {
        let stats = analysis::gamma_exposure::gamma_exposure(&symbol, options).await?;
        Ok(stats)
    }
}

fn default_interval() -> OhlcInterval {
    OhlcInterval::FiveMinute
}
