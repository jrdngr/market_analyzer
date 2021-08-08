use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ohlc {
    pub interval: OhlcInterval,
    pub time: String,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub vwap: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OhlcInterval {
    Tick,
    OneMinute,
    FiveMinute,
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
            _      => anyhow::bail!("Invalid interval: {}", s),
        })
    }
}
