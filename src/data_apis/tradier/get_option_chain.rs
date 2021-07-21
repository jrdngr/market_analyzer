use std::path::Path;

use chrono::Utc;
use serde::{Deserialize, Serialize};

const DATA_PATH: &str = "data";

pub async fn get_option_chain(
    symbol: &str,
    force_download: bool,
) -> anyhow::Result<Vec<OptionInfo>> {
    let file_date = Utc::now().format("%Y%m%d").to_string();
    let file_path = format!("{}/{}_{}.json", DATA_PATH, symbol, file_date);
    let data_path = Path::new(&file_path);

    if data_path.exists() && !force_download {
        log::info!("Fetching cached data for {}", symbol);

        let json = std::fs::read_to_string(&data_path)?;
        Ok(serde_json::from_str(&json)?)
    } else {
        log::info!("Downloading today's data for {}", symbol);
        download_data(symbol, data_path).await
    }
}

async fn download_data(symbol: &str, data_path: &Path) -> anyhow::Result<Vec<OptionInfo>> {
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

    let cache_data = serde_json::to_string_pretty(&result)?;
    std::fs::create_dir_all(DATA_PATH)?;
    std::fs::write(&data_path, &cache_data)?;

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
