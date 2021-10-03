use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Ohlc, OhlcInterval};

pub async fn get_ohlc(symbol: &str, interval: OhlcInterval) -> anyhow::Result<Vec<Ohlc>> {
    get_ohlc_impl(symbol, interval, None).await
}

pub async fn _get_ohlc_authenticated(
    symbol: &str,
    interval: OhlcInterval,
    token: &str,
) -> anyhow::Result<Vec<Ohlc>> {
    get_ohlc_impl(symbol, interval, Some(token)).await
}

async fn get_ohlc_impl(
    symbol: &str,
    interval: OhlcInterval,
    token: Option<&str>,
) -> anyhow::Result<Vec<Ohlc>> {
    let now = Utc::now() - Duration::hours(4);

    let lookback_days = match now.weekday() {
        chrono::Weekday::Sun => 5,
        chrono::Weekday::Sat => 4,
        _ => 3,
    };

    let mut params = format!(
        "period={}&periodType=day&frequency=5&frequencyType=minute",
        lookback_days
    );

    if token.is_none() {
        let api_key = std::env::var(super::API_KEY_ENV)?;
        params.push_str(&format!("&apikey={}", api_key));
    }

    let url = format!(
        "{}/marketdata/{}/pricehistory?{}",
        super::BASE_URL,
        symbol,
        params
    );
    let client = reqwest::Client::new();

    let mut request = client.get(url).header("Accept", "application/json");

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let body = request.send().await?.text().await?;

    let response: OhlcResponse = serde_json::from_str(&body).map_err(|e| {
        log::error!("{}", e);
        log::error!("{}", &body);
        e
    })?;

    let result = response
        .candles
        .into_iter()
        .map(|c| (interval, c).into())
        .collect();

    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OhlcResponse {
    pub symbol: String,
    pub empty: bool,
    pub candles: Vec<Candle>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candle {
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub datetime: Option<i64>,
    pub volume: Option<i64>,
}

impl From<(OhlcInterval, Candle)> for Ohlc {
    fn from((interval, candle): (OhlcInterval, Candle)) -> Self {
        let time = candle.datetime.map(|dt| {
            let seconds = dt / 1000;
            let nanos = (dt % 1000) * 1000;

            let naive = chrono::NaiveDateTime::from_timestamp(seconds, nanos as u32);
            let time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            time.format("%Y-%m-%dT%H:%M:%S").to_string()
        });

        Self {
            interval,
            time,
            open: candle.open,
            high: candle.high,
            low: candle.low,
            close: candle.close,
            volume: candle.volume.map(|v| v as u64),
            vwap: None,
        }
    }
}
