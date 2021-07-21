use serde::Deserialize;

pub async fn get_option_expirations(symbol: &str) -> anyhow::Result<Vec<String>> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let params = format!("symbol={}&includeAllRoots=true", symbol);
    let url = format!("{}/markets/options/expirations?{}", super::BASE_URL, params);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .text()
        .await?;

    let expirations: ExpirationResponse = serde_json::from_str(&body)?;

    Ok(expirations.expirations.ok_or_else(|| anyhow::anyhow!("No expirations"))?.date)
}

#[derive(Clone, Debug, Deserialize)]
struct ExpirationResponse {
    expirations: Option<ExpirationResponseInner>,
}

#[derive(Clone, Debug, Deserialize)]
struct ExpirationResponseInner {
    date: Vec<String>,
}
