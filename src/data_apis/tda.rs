mod get_ohlc;
mod get_quote;
mod options;

pub use get_ohlc::get_ohlc;
pub use get_quote::get_quote;
pub use options::*;

use std::path::Path;

use chrono::Utc;

const DATA_PATH: &str = "data";
const API_KEY_ENV: &str = "API_KEY";
const BASE_URL: &str = "https://api.tdameritrade.com/v1";
const OPTION_CHAIN_URL: &str = "https://api.tdameritrade.com/v1/marketdata/chains";

pub async fn get_option_chain(symbol: &str, force_download: bool) -> anyhow::Result<OptionChain> {
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

async fn download_data(symbol: &str, data_path: &Path) -> anyhow::Result<OptionChain> {
    let api_key = std::env::var(API_KEY_ENV)?;
    let params = format!("apikey={}&symbol={}", api_key, symbol);
    let url = format!("{}?{}", OPTION_CHAIN_URL, params);

    let body = reqwest::get(url).await?.text().await?;

    std::fs::create_dir_all(DATA_PATH)?;
    std::fs::write(&data_path, &body)?;

    let result: OptionChain = serde_json::from_str(&body)?;

    if result.status == "FAILED" {
        anyhow::bail!("Data request failed");
    }

    Ok(result)
}
