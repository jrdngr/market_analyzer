use std::{convert::TryFrom, str::FromStr};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::types;

const DATA_PATH: &str = "data";

pub async fn get_option_chain(
    symbol: &str,
) -> anyhow::Result<Vec<OptionInfo>> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;

    let expirations = super::get_option_expirations(symbol).await?;

    let mut result = Vec::new();

    for expiration in expirations {
        let params = format!("symbol={}&expiration={}&greeks=true", symbol, expiration);
        let url = format!("{}/markets/options/chains?{}", super::BASE_URL, params);

        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .text()
            .await?;

        let response = serde_json::from_str::<OptionChainResponse>(&body)?;
        result.extend(response.options.option);
    }

    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionInfo {
    pub symbol: String,
    pub description: String,
    pub exch: String,
    #[serde(rename = "type")]
    pub symbol_type: String,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub volume: u64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub underlying: String,
    pub strike: f64,
    pub change_percentage: Option<f64>,
    pub average_volume: u64,
    pub last_volume: u64,
    pub trade_date: u64,
    pub prevclose: Option<f64>,
    pub week_52_high: Option<f64>,
    pub week_52_low: Option<f64>,
    pub bidsize: u64,
    pub bidexch: Option<String>,
    pub bid_date: u64,
    pub asksize: u64,
    pub askexch: Option<String>,
    pub ask_date: u64,
    pub open_interest: u64,
    pub contract_size: u64,
    pub expiration_date: String,
    pub expiration_type: String,
    pub option_type: String,
    pub root_symbol: String,
    pub greeks: Option<OptionGreeks>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub phi: f64,
    pub bid_iv: f64,
    pub mid_iv: f64,
    pub ask_iv: f64,
    pub smv_vol: f64,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
struct OptionChainResponse {
    options: OptionChainResponseInner,
}

#[derive(Clone, Debug, Deserialize)]
struct OptionChainResponseInner {
    option: Vec<OptionInfo>,
}

impl TryFrom<OptionInfo> for types::OptionInfo {
    type Error = anyhow::Error;

    fn try_from(info: OptionInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            timestamp: Utc::now().to_rfc3339(),
            symbol: info.root_symbol,
            option_type: types::OptionType::from_str(&info.option_type)?,
            strike: info.strike,
            expiration_date: info.expiration_date,
            open_interest: info.open_interest,
            last: info.last,
            change: info.change,
            volume: info.volume,
            open: info.open,
            high: info.high,
            low: info.low,
            close: info.close,
            delta: info.greeks.as_ref().map(|g| g.delta),
            gamma: info.greeks.as_ref().map(|g| g.gamma),
            theta: info.greeks.as_ref().map(|g| g.theta),
            vega: info.greeks.as_ref().map(|g| g.vega),
            rho: info.greeks.as_ref().map(|g| g.rho),
            bid_iv: info.greeks.as_ref().map(|g| g.bid_iv),
            mid_iv: info.greeks.as_ref().map(|g| g.mid_iv),
            ask_iv: info.greeks.as_ref().map(|g| g.ask_iv),
            smv_vol: info.greeks.as_ref().map(|g| g.smv_vol),
        })
    }
}
