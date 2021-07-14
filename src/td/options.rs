use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Option<Underlying>,
    pub strategy: Strategy,
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub days_to_expiration: f64,
    pub interest_rate: f64,
    pub underlying_price: f64,
    pub volatility: f64,
    pub number_of_contracts: u64,
    pub call_exp_date_map: HashMap<String, HashMap<Decimal, Vec<OptionData>>>,
    pub put_exp_date_map: HashMap<String, HashMap<Decimal, Vec<OptionData>>>,
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
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: f64,
    pub quote_time_in_long: u64,
    pub trade_time_in_long: u64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub time_value: f64,
    pub open_interest: f64,
    pub is_in_the_money: Option<bool>,
    pub theoretical_option_value: f64,
    pub theoretical_volatility: f64,
    pub is_mini: Option<bool>,
    pub is_non_standard: Option<bool>,
    pub option_deliverables_list: Option<Vec<OptionDeliverables>>,
    pub strike_price: f64,
    pub expiration_date: u64,
    pub days_to_expiration: u64,
    pub expiration_type: String,
    pub last_trading_day: u64,
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: Option<bool>,
    pub percent_change: f64,
    pub mark_change: f64,
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
    pub ask: f64,
    pub ask_size: f64,
    pub bid: f64,
    pub bid_size: f64,
    pub change: f64,
    pub close: f64,
    pub delayed: bool,
    pub description: String,
    pub exchange_name: ExchangeName,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_low: f64,
    pub high_price: f64,
    pub last: f64,
    pub low_price: f64,
    pub mark: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub open_price: f64,
    pub percent_change: f64,
    pub quote_time: f64,
    pub symbol: String,
    pub total_volume: f64,
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
