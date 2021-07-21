use serde::{Serialize, Deserialize};

pub async fn get_quote(symbol: &str) -> anyhow::Result<Quote> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let params = format!("symbols=${}", symbol);
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

    let quotes: Quotes = serde_json::from_str(&body)?;
    let quote = quotes.quotes.quote.into_iter().next().ok_or_else(|| anyhow::anyhow!("No quote"))?;

    Ok(quote)
}

#[derive(Clone, Debug, Deserialize)]
struct Quotes {
    quotes: Quote2,
}

#[derive(Clone, Debug, Deserialize)]
struct Quote2 {
    quote: Vec<Quote>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub description: String,
    pub exch: String,
    #[serde(rename = "type")]
    pub quote_type: String,
    pub last: f64,
    pub change: f64,
    pub volume: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: Option<f64>,
    pub bid: f64,
    pub ask: f64,
    pub change_percentage: f64,
    pub average_volume: u64,
    pub last_volume: u64,
    pub trade_date: u64,
    pub prevclose: f64,
    pub week_52_high: f64,
    pub week_52_low: f64,
    pub bidsize: u64,
    pub bidexch: String,
    pub bid_date: u64,
    pub asksize: u64,
    pub askexch: String,
    pub ask_date: u64,
    pub root_symbols: Vec<String>,
}
