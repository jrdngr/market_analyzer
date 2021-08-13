use std::str::FromStr;

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct OptionInfo {
    pub timestamp: String,
    pub symbol: String,
    pub option_type: OptionType,
    pub strike: f64,
    pub expiration_date: String,
    pub open_interest: u64,
    pub volume: u64,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
    pub bid_iv: Option<f64>,
    pub mid_iv: Option<f64>,
    pub ask_iv: Option<f64>,
    pub smv_vol: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum OptionType {
    Call,
    Put,
}

impl FromStr for OptionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "call" | "c" => OptionType::Call,
            "put" | "p" => OptionType::Put,
            _ => anyhow::bail!("Invalid option type: {}", s),
        })
    }
}

#[cfg(test)]
impl OptionInfo {
    pub fn test() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            symbol: "TST".to_string(),
            option_type: OptionType::Call,
            strike: 0.0,
            expiration_date: "tomorrow".to_string(),
            open_interest: 2,
            volume: 5,
            last: Some(3.0),
            change: Some(4.0),
            open: Some(6.0),
            high: Some(7.0),
            low: Some(8.0),
            close: Some(9.0),
            delta: Some(10.0),
            gamma: Some(11.0),
            theta: Some(12.0),
            vega: Some(13.0),
            rho: Some(14.0),
            bid_iv: Some(15.0),
            mid_iv: Some(18.0),
            ask_iv: Some(16.0),
            smv_vol: Some(17.0),
        }
    }
}
