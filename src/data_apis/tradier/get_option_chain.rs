use std::str::FromStr;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{math::bs, types};

pub async fn get_option_chain(symbol: &str) -> anyhow::Result<Vec<types::OptionInfo>> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;

    let expirations = super::get_option_expirations(symbol).await?;
    let quote = super::get_quote(symbol).await?;
    let current_price = quote.last.unwrap_or(0.0);

    let mut option_info = Vec::new();

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

        let response = serde_json::from_str::<OptionChainResponse>(&body).map_err(|e| {
            log::error!("{}", e);
            log::error!("{}", &body);
            e
        })?;
        option_info.extend(response.options.option);
    }

    let mut result = Vec::new();
    for oi in option_info {
        result.push(oi.into_crate_type(current_price).await?);
    }

    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OptionInfo {
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

impl OptionInfo {
    pub async fn into_crate_type(self, current_price: f64) -> anyhow::Result<types::OptionInfo> {
        let option_type = types::OptionType::from_str(&self.option_type)?;

        let greeks = match &self.greeks {
            Some(g) => {
                let expiration_time = 180.0;
                let current_time = 0.0;

                let delta = match option_type {
                    types::OptionType::Call => bs::call_delta,
                    types::OptionType::Put => bs::put_delta,
                };

                Some(types::Greeks {
                    delta: delta(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                    gamma: bs::gamma(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                    theta: bs::theta(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                    vega: bs::vega(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                    rho: g.rho,
                    vanna: bs::vanna(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                    charm: bs::charm(
                        g.mid_iv,
                        expiration_time,
                        current_time,
                        current_price,
                        self.strike,
                    ),
                })
            }
            None => None,
        };

        Ok(types::OptionInfo {
            timestamp: Utc::now().to_rfc3339(),
            symbol: self.root_symbol,
            option_type,
            strike: self.strike,
            expiration_date: self.expiration_date,
            open_interest: self.open_interest,
            last: self.last,
            change: self.change,
            volume: self.volume,
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            greeks,
            bid_iv: self.greeks.as_ref().map(|g| g.bid_iv),
            mid_iv: self.greeks.as_ref().map(|g| g.mid_iv),
            ask_iv: self.greeks.as_ref().map(|g| g.ask_iv),
            smv_vol: self.greeks.as_ref().map(|g| g.smv_vol),
        })
    }
}
