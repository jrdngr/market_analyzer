use std::str::FromStr;

use chrono::{Datelike, Duration, Local};
use serde::{Deserialize, Serialize};

pub async fn get_time_and_sales(symbol: &str, interval: &str) -> anyhow::Result<Vec<TimeAndSales>> {
    let interval = TimeAndSalesInterval::from_str(interval)?;

    let lookback_days = match Local::now().weekday() {
        chrono::Weekday::Sun => 3,
        chrono::Weekday::Sat => 2,
        _ => 1,
    };
    let start = (Local::now() - Duration::days(lookback_days)).format("%Y-%m-%d %H:%M").to_string();
    
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

    let time_and_sales: TimeAndSalesResponse = serde_json::from_str(&body)?;

    Ok(time_and_sales.series.data)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TimeAndSalesInterval {
    Tick,
    OneMinute,
    FiveMinute,
    FifteenMinutes,
}

impl std::fmt::Display for TimeAndSalesInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TimeAndSalesInterval::*;

        let s = match self {
            Tick => "tick",
            OneMinute => "1min",
            FiveMinute => "5min",
            FifteenMinutes => "15min",
        };

        write!(f, "{}", s)
    }
}

impl FromStr for TimeAndSalesInterval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "tick" => TimeAndSalesInterval::Tick,
            "1min" => TimeAndSalesInterval::OneMinute,
            "5min" => TimeAndSalesInterval::FiveMinute,
            "15min" => TimeAndSalesInterval::FifteenMinutes,
            _      => anyhow::bail!("Invalid interval: {}", s),
        })
    }
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
    pub vwap: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct TimeAndSalesResponse {
    series: TimeAndSalesInner,
}

#[derive(Clone, Debug, Deserialize)]
struct TimeAndSalesInner {
    data: Vec<TimeAndSales>,
}
