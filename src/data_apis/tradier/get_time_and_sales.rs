use chrono::{Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{self as graphql, OhlcInterval};

pub async fn get_time_and_sales(
    symbol: &str,
    interval: OhlcInterval,
) -> anyhow::Result<Vec<graphql::Ohlc>> {
    let now = Utc::now() - Duration::hours(4);

    let lookback_days = match now.weekday() {
        chrono::Weekday::Sun => 5,
        chrono::Weekday::Sat => 4,
        _ => 3,
    };
    let start = (now - Duration::days(lookback_days))
        .format("%Y-%m-%d %H:%M")
        .to_string();

    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let params = format!("symbol={}&interval={}&start={}", symbol, interval, start);
    let url = format!("{}/markets/timesales?{}", super::BASE_URL, params);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .text()
        .await?;

    let time_and_sales: TimeAndSalesResponse = serde_json::from_str(&body).map_err(|e| {
        log::error!("{}", e);
        log::error!("{}", &body);
        e
    })?;

    let data = time_and_sales.series.unwrap_or_default().data;
    let result = data.into_iter().map(|ts| (interval, ts).into()).collect();
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeAndSales {
    pub time: String,
    pub timestamp: u64,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub vwap: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct TimeAndSalesResponse {
    series: Option<TimeAndSalesInner>,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct TimeAndSalesInner {
    data: Vec<TimeAndSales>,
}

impl From<(graphql::OhlcInterval, TimeAndSales)> for graphql::Ohlc {
    fn from((interval, ts): (graphql::OhlcInterval, TimeAndSales)) -> Self {
        Self {
            interval,
            time: Some(ts.time),
            open: Some(ts.open),
            high: Some(ts.high),
            low: Some(ts.low),
            close: Some(ts.close),
            volume: Some(ts.volume),
            vwap: ts.vwap,
        }
    }
}
