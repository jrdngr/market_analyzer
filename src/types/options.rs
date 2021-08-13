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
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub volume: u64,
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
