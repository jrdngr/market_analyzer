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
    pub greeks: Option<Greeks>,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub bid_iv: Option<f64>,
    pub mid_iv: Option<f64>,
    pub ask_iv: Option<f64>,
    pub smv_vol: Option<f64>,
}

impl OptionInfo {
    pub fn delta(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.delta,
            None => 0.0,
        }
    }

    pub fn gamma(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.gamma,
            None => 0.0,
        }
    }

    pub fn theta(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.theta,
            None => 0.0,
        }
    }

    pub fn vega(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.vega,
            None => 0.0,
        }
    }

    pub fn rho(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.rho,
            None => 0.0,
        }
    }

    pub fn vanna(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.vanna,
            None => 0.0,
        }
    }

    pub fn charm(&self) -> f64 {
        match &self.greeks {
            Some(g) => g.charm,
            None => 0.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub vanna: f64,
    pub charm: f64,
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
            greeks: Some(Greeks {
                delta: 10.0,
                gamma: 11.0,
                theta: 12.0,
                vega: 13.0,
                rho: 14.0,
                vanna: 15.0,
                charm: 16.0,
            }),
            last: Some(3.0),
            change: Some(4.0),
            open: Some(6.0),
            high: Some(7.0),
            low: Some(8.0),
            close: Some(9.0),
            bid_iv: Some(15.0),
            mid_iv: Some(18.0),
            ask_iv: Some(16.0),
            smv_vol: Some(17.0),
        }
    }
}
