use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GammaExposure {
    pub strike: String,
    pub gamma_exposure: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GammaExposureStats {
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
