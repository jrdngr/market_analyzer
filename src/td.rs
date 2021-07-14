mod options;

pub use options::*;

use std::path::Path;

use chrono::Utc;

const FILE_PATH: &str = "data";
const API_KEY_ENV: &str = "API_KEY";
const OPTION_CHAIN_URL: &str = "https://api.tdameritrade.com/v1/marketdata/chains";

pub async fn get_option_chain(symbol: &str) -> anyhow::Result<OptionChain> {
    let file_date = Utc::now().format("%Y%m%d").to_string();
    let file_path = format!("{}/{}_{}.json", FILE_PATH, symbol, file_date);
    let path = Path::new(&file_path);

    let body = if path.exists() {
        std::fs::read_to_string(&path)?
    } else {
        println!("Downloading today's {} data", symbol);
        let api_key = std::env::var(API_KEY_ENV)?;
        let params = format!("apikey={}&symbol={}", api_key, symbol);
        let url = format!("{}?{}", OPTION_CHAIN_URL, params);

        let body = reqwest::get(url).await?.text().await?;

        std::fs::create_dir_all(FILE_PATH)?;
        std::fs::write(&path, &body)?;

        body
    };

    let result = serde_json::from_str(&body)?;

    Ok(result)
}
