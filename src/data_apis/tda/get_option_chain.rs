use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{math::bs, types, utils::deserialize_f64_with_nan};

pub async fn get_option_chain(symbol: &str) -> anyhow::Result<Vec<types::OptionInfo>> {
    get_option_chain_impl(symbol, None).await
}

pub async fn _get_option_chain_authenticated(
    symbol: &str,
    token: &str,
) -> anyhow::Result<Vec<types::OptionInfo>> {
    get_option_chain_impl(symbol, Some(token)).await
}

async fn get_option_chain_impl(
    symbol: &str,
    token: Option<&str>,
) -> anyhow::Result<Vec<types::OptionInfo>> {
    let mut params = format!("symbol={}", symbol);

    if token.is_none() {
        let api_key = std::env::var(super::API_KEY_ENV)?;
        params.push_str(&format!("&apikey={}", api_key));
    }

    let url = format!("{}/marketdata/chains?{}", super::BASE_URL, params);

    let client = reqwest::Client::new();

    let mut request = client.get(url).header("Accept", "application/json");

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let body = request.send().await?.text().await?;

    let chain: OptionChain = serde_json::from_str(&body).map_err(|e| {
        log::error!("{}", e);
        log::error!("{}", &body);
        e
    })?;

    let mut result = Vec::new();
    let price = chain.underlying_price;

    for (_, map) in chain.call_exp_date_map.into_iter() {
        for (_, options) in map.into_iter() {
            result.extend(options.into_iter().map(|o| o.into_crate_type(price)));
        }
    }

    for (_, map) in chain.put_exp_date_map.into_iter() {
        for (_, options) in map.into_iter() {
            result.extend(options.into_iter().map(|o| o.into_crate_type(price)));
        }
    }

    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Option<Underlying>,
    pub strategy: Strategy,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub days_to_expiration: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub interest_rate: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub underlying_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub volatility: f64,
    pub number_of_contracts: u64,
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionData>>>,
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionData>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Strategy {
    Single,
    Analytical,
    Covered,
    Vertical,
    Calendar,
    Strangle,
    Straddle,
    Butterfly,
    Condor,
    Diagonal,
    Collar,
    Roll,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PutOrCall {
    Put,
    Call,
}

impl From<PutOrCall> for types::OptionType {
    fn from(pc: PutOrCall) -> Self {
        match pc {
            PutOrCall::Put => types::OptionType::Put,
            PutOrCall::Call => types::OptionType::Call,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionData {
    pub put_call: PutOrCall,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub last_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub bid_size: u64,
    pub ask_size: u64,
    pub last_size: u64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub high_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub low_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub open_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub close_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub total_volume: f64,
    pub quote_time_in_long: u64,
    pub trade_time_in_long: u64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub net_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub volatility: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub delta: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub gamma: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub theta: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub vega: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub rho: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub time_value: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub open_interest: f64,
    pub is_in_the_money: Option<bool>,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub theoretical_option_value: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub theoretical_volatility: f64,
    pub is_mini: Option<bool>,
    pub is_non_standard: Option<bool>,
    pub option_deliverables_list: Option<Vec<OptionDeliverables>>,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub strike_price: f64,
    pub expiration_date: i64,
    pub days_to_expiration: i64,
    pub expiration_type: String,
    pub last_trading_day: u64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: Option<bool>,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub percent_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub mark_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub mark_percent_change: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum ExchangeName {
    IND,
    ASE,
    NYS,
    NAS,
    NAP,
    PAC,
    OPR,
    BATS,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub ask: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub ask_size: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub bid: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub bid_size: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub close: f64,
    pub delayed: bool,
    pub description: String,
    pub exchange_name: ExchangeName,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub fifty_two_week_high: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub fifty_two_week_low: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub high_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub last: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub low_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub mark: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub mark_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub mark_percent_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub open_price: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub percent_change: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub quote_time: f64,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub total_volume: f64,
    #[serde(deserialize_with = "deserialize_f64_with_nan")]
    pub trade_time: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverables {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: String,
    pub currency_type: String,
}

impl OptionData {
    pub fn into_crate_type(self, current_price: f64) -> types::OptionInfo {
        let option_type = self.put_call.into();

        let expiration_time = 180.0;
        let current_time = 0.0;

        let delta = match option_type {
            types::OptionType::Call => bs::call_delta,
            types::OptionType::Put => bs::put_delta,
        };

        let volatility = self.volatility / 100.0;

        let greeks = types::Greeks {
            delta: delta(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
            gamma: bs::gamma(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
            theta: bs::theta(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
            vega: bs::vega(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
            rho: self.rho,
            vanna: bs::vanna(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
            charm: bs::charm(
                volatility,
                expiration_time,
                current_time,
                current_price,
                self.strike_price,
            ),
        };

        let seconds = self.expiration_date / 1000;
        let nanos = (self.expiration_date % 1000) * 1000;

        let naive = chrono::NaiveDateTime::from_timestamp(seconds, nanos as u32);
        let time: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let expiration_date = time.format("%Y-%m-%dT%H:%M:%S").to_string();

        types::OptionInfo {
            timestamp: Utc::now().to_rfc3339(),
            symbol: self.symbol,
            option_type,
            strike: self.strike_price,
            expiration_date,
            open_interest: self.open_interest as u64,
            last: self.last_price,
            change: Some(self.net_change),
            volume: self.total_volume as u64,
            open: Some(self.open_price),
            high: Some(self.high_price),
            low: Some(self.low_price),
            close: Some(self.close_price),
            greeks: Some(greeks),
            bid_iv: None,
            mid_iv: Some(volatility),
            ask_iv: None,
            smv_vol: None,
        }
    }
}
