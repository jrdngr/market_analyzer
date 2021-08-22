use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clock {
    pub timestamp: u64,
    pub date: String,
    pub description: String,
    pub state: MarketState,
    pub next_state: MarketState,
    pub next_change_minutes: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketState {
    PreMarket,
    Open,
    PostMarket,
    Closed,
}

impl FromStr for MarketState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "premarket" => Ok(MarketState::PreMarket),
            "open" => Ok(MarketState::Open),
            "postmarket" => Ok(MarketState::PostMarket),
            "closed" => Ok(MarketState::Closed),
            _ => anyhow::bail!("Invalid market state {}", s),
        }
    }
}

impl Display for MarketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketState::PreMarket => write!(f, "Pre-Market"),
            MarketState::Open => write!(f, "Open"),
            MarketState::PostMarket => write!(f, "Post-Market"),
            MarketState::Closed => write!(f, "Closed"),
        }
    }
}
