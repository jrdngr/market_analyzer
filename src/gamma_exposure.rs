use std::{collections::BTreeMap, convert::TryFrom};

use rust_decimal::{Decimal, prelude::FromPrimitive};
use serde::{Deserialize, Serialize};

use crate::data_apis::tradier;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GammaExposureStats {
    pub prices: Vec<GammaExposure>,
    pub average_absolute_exposure: f64,
    pub average_positive_exposure: f64,
    pub average_negative_exposure: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub absolute_maximum: f64,
    pub weighted_average_absolute_price: f64,
    pub weighted_average_positive_price: f64,
    pub weighted_average_negative_price: f64,
}

impl GammaExposureStats {
    pub fn new(strike_to_gamma_exposure: &BTreeMap<Decimal, f64>) -> anyhow::Result<Self> {
        let mut positive_sum: f64 = 0.0;
        let mut positive_count = 0;
        let mut negative_sum: f64 = 0.0;
        let mut negative_count = 0;
        let mut maximum: f64 = 0.0;
        let mut minimum: f64 = 0.0;
        let mut absolute_maximum: f64 = 0.0;
        let mut weighted_positive_sum = Decimal::ZERO;
        let mut weighted_negative_sum = Decimal::ZERO;

        for (strike, exposure) in strike_to_gamma_exposure {
            if *exposure >= 0.0 {
                positive_sum += exposure;
                weighted_positive_sum += strike * Decimal::try_from(*exposure)?;
                positive_count += 1;
            } else {
                negative_sum += exposure;
                weighted_negative_sum += strike * Decimal::try_from(*exposure)?;
                negative_count += 1;
            }
            maximum = maximum.max(*exposure);
            minimum = minimum.min(*exposure);
            absolute_maximum = absolute_maximum.max(exposure.abs());
        }

        positive_count = positive_count.max(1);
        negative_count = negative_count.max(1);

        let average_positive_exposure = positive_sum / positive_count as f64;
        let average_negative_exposure = negative_sum / negative_count as f64;
        let average_absolute_exposure =
            (positive_sum.abs() + negative_sum.abs()) / (positive_count + negative_count) as f64;

        let weighted_average_absolute_price =
            f64::try_from(weighted_positive_sum.abs() + weighted_negative_sum.abs())?
                / (positive_sum.abs() + negative_sum.abs());
        let weighted_average_positive_price = f64::try_from(weighted_positive_sum)? / positive_sum;
        let weighted_average_negative_price = f64::try_from(weighted_negative_sum)? / negative_sum;

        let mut prices: Vec<GammaExposure> = strike_to_gamma_exposure
            .into_iter()
            .map(|(strike, exposure)| GammaExposure::new(*strike, *exposure))
            .collect();

        prices.sort_by_key(|k| k.strike);

        Ok(Self {
            prices,
            average_absolute_exposure,
            average_positive_exposure,
            average_negative_exposure,
            maximum,
            minimum,
            absolute_maximum,
            weighted_average_absolute_price,
            weighted_average_positive_price,
            weighted_average_negative_price,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GammaExposure {
    pub strike: Decimal,
    pub gamma_exposure: f64,
}

impl GammaExposure {
    pub fn new(strike: Decimal, gamma_exposure: f64) -> Self {
        Self {
            strike,
            gamma_exposure,
        }
    }
}

pub async fn gamma_exposure_by_price(
    symbol: &str,
    force_download: bool,
) -> anyhow::Result<GammaExposureStats> {
    dotenv::dotenv()?;

    let options = tradier::get_option_chain(&symbol.to_uppercase(), force_download).await?;

    let mut strike_to_gamma_exposure: BTreeMap<Decimal, f64> = BTreeMap::new();

    for option in options {
        let strike = Decimal::from_f64(option.strike);
        if let (Some(strike), Some(greeks)) = (strike, option.greeks) {
            let mut exposure = greeks.gamma * option.open_interest as f64;
            if option.option_type == "put" {
                exposure *= -1.0;
            }
            match strike_to_gamma_exposure.get_mut(&strike) {
                Some(exp) => *exp += exposure,
                None => {
                    strike_to_gamma_exposure.insert(strike, exposure);
                }
            }    
        }
    }

    Ok(GammaExposureStats::new(&strike_to_gamma_exposure)?)
}
