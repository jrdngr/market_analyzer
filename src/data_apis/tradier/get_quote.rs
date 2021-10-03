use crate::types;
use serde::{Deserialize, Serialize};

pub async fn get_quote(symbol: &str) -> anyhow::Result<Quote> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let params = format!("symbols={}", symbol);
    let url = format!("{}/markets/quotes?{}", super::BASE_URL, params);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .text()
        .await?;

    let quotes: QuoteResponse = serde_json::from_str(&body).map_err(|e| {
        log::error!("{}", e);
        log::error!("{}", &body);
        e
    })?;

    Ok(quotes.quotes.quote)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub description: String,
    pub exch: String,
    #[serde(rename = "type")]
    pub quote_type: String,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub volume: u64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
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
    pub root_symbol: Option<String>,
    pub root_symbols: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct QuoteResponse {
    quotes: QuoteResponseInner,
}

#[derive(Clone, Debug, Deserialize)]
struct QuoteResponseInner {
    quote: Quote,
}

impl From<Quote> for types::Quote {
    fn from(quote: Quote) -> Self {
        Self {
            symbol: quote.symbol,
            last: quote.last,
            change: quote.change,
            volume: Some(quote.volume),
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
        }
    }
}
