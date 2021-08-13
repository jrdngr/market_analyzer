use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, InputObject)]
#[serde(default)]
pub struct GammaExposureOptions {
    #[graphql(default)]
    pub aggregate: bool,
    #[graphql(default)]
    pub fresh: bool,
    pub min_strike: Option<f64>,
    pub max_strike: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct GammaExposure {
    pub strike: String,
    pub gamma_exposure: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct GammaExposureStats {
    pub timestamp: String,
    pub symbol: String,
    pub prices: Vec<GammaExposure>,
    pub average_absolute_exposure: f64,
    pub average_positive_exposure: f64,
    pub average_negative_exposure: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub absolute_maximum: f64,
    pub absolute_minimum: f64,
    pub weighted_average_absolute_price: f64,
    pub weighted_average_positive_price: f64,
    pub weighted_average_negative_price: f64,
    pub absolute_maximum_price: f64,
    pub absolute_minimum_price: f64,
}

#[cfg(test)]
impl GammaExposureStats {
    pub fn test() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            symbol: "TST".to_string(),
            prices: vec![
                GammaExposure {
                    strike: "1.0".to_string(),
                    gamma_exposure: 1.0,
                },
                GammaExposure {
                    strike: "2.0".to_string(),
                    gamma_exposure: 2.0,
                },
                GammaExposure {
                    strike: "3.0".to_string(),
                    gamma_exposure: 3.0,
                },
            ],
            average_absolute_exposure: 1.0,
            average_positive_exposure: 2.0,
            average_negative_exposure: 3.0,
            maximum: 4.0,
            minimum: 5.0,
            absolute_maximum: 6.0,
            absolute_minimum: 7.0,
            weighted_average_absolute_price: 8.0,
            weighted_average_positive_price: 9.0,
            weighted_average_negative_price: 10.0,
            absolute_maximum_price: 11.0,
            absolute_minimum_price: 12.0,
        }
    }
}
