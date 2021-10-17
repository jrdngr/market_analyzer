use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgMoney;

use crate::types::OptionType;

#[derive(Clone, Debug)]
pub struct OptionInfo {
    pub id: i32,
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub option_type: String,
    pub strike: PgMoney,
    pub expiration_date: DateTime<Utc>,
    pub open_interest: Option<i64>,
    pub volume: Option<i64>,
    pub last: Option<PgMoney>,
    pub open: Option<PgMoney>,
    pub high: Option<PgMoney>,
    pub low: Option<PgMoney>,
    pub close: Option<PgMoney>,
    pub change: Option<f64>,
    pub underlying_price: Option<PgMoney>,
    pub implied_volatility: Option<f64>,
}
