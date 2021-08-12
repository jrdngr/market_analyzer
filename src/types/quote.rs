use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Quote {
    pub symbol: String,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub volume: u64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
}
