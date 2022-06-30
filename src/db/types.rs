use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::types::OptionType;

#[derive(Clone, Debug)]
pub struct OptionInfo {
    pub id: i32,
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub option_type: String,
    pub strike: Decimal,
    pub expiration_date: DateTime<Utc>,
    pub open_interest: Option<i64>,
    pub volume: Option<i64>,
    pub last: Option<Decimal>,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Option<Decimal>,
    pub change: Option<f64>,
    pub underlying_price: Option<Decimal>,
    pub implied_volatility: Option<f64>,
}
