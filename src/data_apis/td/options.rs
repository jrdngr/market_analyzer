use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::deserialize_f64_with_nan;

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
