use std::str::FromStr;

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Ohlc {
    pub interval: OhlcInterval,
    pub time: Option<String>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<u64>,
    pub vwap: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum OhlcInterval {
    #[graphql(name = "tick")]
    Tick,
    #[graphql(name = "1min")]
    OneMinute,
    #[graphql(name = "5min")]
    FiveMinute,
    #[graphql(name = "15min")]
    FifteenMinute,
}

impl std::fmt::Display for OhlcInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OhlcInterval::*;

        let s = match self {
            Tick => "tick",
            OneMinute => "1min",
            FiveMinute => "5min",
            FifteenMinute => "15min",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for OhlcInterval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "tick" => OhlcInterval::Tick,
            "1min" => OhlcInterval::OneMinute,
            "5min" => OhlcInterval::FiveMinute,
            "15min" => OhlcInterval::FifteenMinute,
            _ => anyhow::bail!("Invalid interval: {}", s),
        })
    }
}
