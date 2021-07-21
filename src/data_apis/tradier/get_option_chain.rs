use serde::{Serialize, Deserialize};

pub async fn get_option_chain(symbol: &str, force_download: bool) -> anyhow::Result<String> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let params = format!("symbol={}&expiration={}&greeks=true", symbol, "");
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

    Ok(body)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionChain {

}