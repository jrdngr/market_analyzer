use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct StrikeStats {
    pub strike: f64,
    pub open_interest: u64,
    pub call_exposure: HedgeExposure,
    pub put_exposure: HedgeExposure,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct HedgeExposure {
    pub gamma: f64,
    pub vanna: f64,
    pub charm: f64,
}
