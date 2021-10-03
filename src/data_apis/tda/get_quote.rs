use std::collections::HashMap;

use crate::types::Quote;
use serde::{Deserialize, Serialize};

pub async fn get_quote(symbol: &str) -> anyhow::Result<Quote> {
    get_quote_impl(symbol, None).await
}

pub async fn get_quote_authenticated(symbol: &str, token: &str) -> anyhow::Result<Quote> {
    get_quote_impl(symbol, Some(token)).await
}

async fn get_quote_impl(symbol: &str, token: Option<&str>) -> anyhow::Result<Quote> {
    let mut url = format!("{}/marketdata/{}/quotes", super::BASE_URL, symbol);
    if token.is_none() {
        let api_key = std::env::var(super::API_KEY_ENV)?;
        url = format!("{}?apikey={}", url, api_key);
    }

    let client = reqwest::Client::new();

    let mut request = client.get(url).header("Accept", "application/json");

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let body = request.send().await?.text().await?;

    let quotes: QuoteResponse = serde_json::from_str(&body).map_err(|e| {
        log::error!("{}", e);
        log::error!("{}", &body);
        e
    })?;

    let result = quotes
        .get(symbol)
        .ok_or_else(|| anyhow::anyhow!("{} not in result", symbol))?
        .clone();

    Ok(result.into())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRaw {
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub exchange_name: String,
    pub net_change: Option<f64>,
    pub total_volume: Option<u64>,
    pub trade_time_in_long: Option<i64>,
    pub digits: Option<i32>,
    #[serde(rename = "52WkHigh")]
    pub high_52_wk: Option<f64>,
    #[serde(rename = "52WkLow")]
    pub low_52_wk: Option<f64>,
    #[serde(rename = "nAV")]
    pub nav: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub div_amount: Option<f64>,
    pub div_yield: Option<f64>,
    pub div_date: Option<String>,
    pub security_status: Option<String>,
    pub bid_price_in_double: Option<f64>,
    pub ask_price_in_double: Option<f64>,
    pub last_price_in_double: Option<f64>,
    pub high_price_in_double: Option<f64>,
    pub low_price_in_double: Option<f64>,
    pub close_price_in_double: Option<f64>,
    pub open_price_in_double: Option<f64>,
    pub change_in_double: Option<f64>,
    pub future_percent_change: Option<f64>,
    pub bid_id: Option<String>,
    pub ask_id: Option<String>,
    pub last_id: Option<String>,
    pub open_interest: Option<f64>,
    pub mark: Option<f64>,
    pub tick: Option<f64>,
    pub tick_amount: Option<f64>,
    // Futures
    pub product: Option<String>,
    pub future_price_format: Option<String>,
    pub future_trading_hours: Option<String>,
    pub future_is_tradable: Option<bool>,
    pub future_multiplier: Option<f64>,
    pub future_is_active: Option<bool>,
    pub future_settlement_price: Option<f64>,
    pub future_expiration_date: Option<f64>,
    // Future Options
    pub volatility: Option<f64>,
    pub money_intrinsic_value_in_double: Option<f64>,
    pub multiplier_in_double: Option<f64>,
    pub strike_price_in_double: Option<f64>,
    pub time_value_in_double: Option<f64>,
    pub delta_value_in_double: Option<f64>,
    pub gamma_value_in_double: Option<f64>,
    pub theta_value_in_double: Option<f64>,
    pub vega_value_in_double: Option<f64>,
    pub rho_value_in_double: Option<f64>,
    pub contract_type: Option<String>,
    pub underlying: Option<String>,
    pub in_the_money: Option<bool>,
    // Index
    pub close_price: Option<f64>,
    pub last_price: Option<f64>,
    pub open_price: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    // Option
    pub bid_price: Option<f64>,
    pub bid_size: Option<i32>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i32>,
    pub last_size: Option<f64>,
    pub quote_time_in_long: Option<i64>,
    pub deliverables: Option<String>,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
    pub theoretical_option_value: Option<f64>,
    pub underlying_price: Option<f64>,
    pub uv_expiration_type: Option<String>,
    pub settlement_type: Option<String>,
    // ETF
    pub marginable: Option<bool>,
    pub shortable: Option<bool>,
    pub regular_market_last_price: Option<f64>,
    pub regular_market_last_size: Option<i32>,
    pub regular_market_net_change: Option<f64>,
    pub regular_market_trade_time_in_long: Option<i64>,
}

type QuoteResponse = HashMap<String, QuoteRaw>;

impl From<QuoteRaw> for Quote {
    fn from(quote: QuoteRaw) -> Self {
        let last = quote
            .last_price
            .or(quote.regular_market_last_price)
            .or(quote.last_price_in_double);
        let change = quote
            .net_change
            .or(quote.regular_market_net_change)
            .or(quote.change_in_double);
        let volume = quote.total_volume;
        let open = quote.open_price.or(quote.open_price_in_double);
        let high = quote.high_price.or(quote.high_price_in_double);
        let low = quote.low_price.or(quote.low_price_in_double);
        let close = quote.close_price.or(quote.close_price_in_double);

        Self {
            symbol: quote.symbol,
            last,
            change,
            volume,
            open,
            high,
            low,
            close,
        }
    }
}
