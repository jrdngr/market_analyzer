use std::{collections::BTreeMap, path::Path};

use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::td;

const REPORT_PATH: &str = "reports";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GammaExposureStats {
    pub prices: Vec<GammaExposure>,
    pub average_absolute_exposure: f64,
    pub average_positive_exposure: f64,
    pub average_negative_exposure: f64,
    pub maximum: f64,
    pub minimum: f64,
    pub absolute_maximum: f64,
}

impl GammaExposureStats {
    pub fn new(strike_to_gamma_exposure: &BTreeMap<Decimal, f64>) -> Self {

        let mut positive_sum: f64 = 0.0;
        let mut positive_count = 0;
        let mut negative_sum: f64 = 0.0;
        let mut negative_count = 0;
        let mut maximum: f64 = 0.0;
        let mut minimum: f64 = 0.0;
        let mut absolute_maximum: f64 = 0.0;

        for (_, exposure) in strike_to_gamma_exposure {
            if *exposure >= 0.0 {
                positive_sum += exposure;
                positive_count += 1;
            } else {
                negative_sum += exposure;
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
        let average_absolute_exposure = (positive_sum.abs() + negative_sum.abs()) / (positive_count + negative_count) as f64;

        let mut prices: Vec<GammaExposure> = strike_to_gamma_exposure
            .into_iter()
            .map(|(strike, exposure)| GammaExposure::new(*strike, *exposure))
            .collect();

        prices.sort_by_key(|k| k.strike);

        Self {
            prices,
            average_absolute_exposure,
            average_positive_exposure,
            average_negative_exposure,
            maximum,
            minimum,
            absolute_maximum,
        }
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

    let options = td::get_option_chain(&symbol.to_uppercase(), force_download).await?;

    let mut strike_to_gamma_exposure: BTreeMap<Decimal, f64> = BTreeMap::new();

    for contracts in options.call_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest;
                match strike_to_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        strike_to_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }
    for contracts in options.put_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest * -1.0;
                match strike_to_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        strike_to_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }

    Ok(GammaExposureStats::new(&strike_to_gamma_exposure))
}

#[deprecated]
/// Original function to generate csv files
pub async fn print_gamma_exposure_by_price(
    symbol: &str,
    force_download: bool,
) -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let options = td::get_option_chain(symbol, force_download).await?;

    let mut call_exposure = 0.0;
    let mut call_count = 0;
    let mut put_exposure = 0.0;
    let mut put_count = 0;
    let mut strike_to_gamma_exposure: BTreeMap<Decimal, f64> = BTreeMap::new();

    for contracts in options.call_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest;
                call_exposure += exposure;
                call_count += 1;
                match strike_to_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        strike_to_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }
    for contracts in options.put_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest * -1.0;
                put_exposure += exposure;
                put_count += 1;
                match strike_to_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        strike_to_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }

    let total_exposure = call_exposure + put_exposure;
    let average_call_exposure = call_exposure / (call_count as f64);
    let average_put_exposure = put_exposure / (put_count as f64);
    let average_exposure =
        (call_exposure.abs() + put_exposure.abs()) / (strike_to_gamma_exposure.len() as f64);

    println!("----------------------------------------------------");
    let mut csv = String::new();
    for (strike, exposure) in strike_to_gamma_exposure {
        if exposure.abs() >= average_exposure {
            csv.push_str(&format!("{}, {}\n", strike, exposure));
            println!("{}: {}", strike, exposure);
        }
    }
    std::fs::create_dir_all(REPORT_PATH)?;
    let file_date = Utc::now().format("%Y%m%d").to_string();
    let file_path = format!("{}/{}_{}.csv", REPORT_PATH, symbol, file_date);
    let path = Path::new(&file_path);

    std::fs::write(&path, csv)?;

    println!("----------------------------------------------------");
    println!("Average call exposure: {}", average_call_exposure);
    println!("Average put exposure: {}", average_put_exposure);
    println!("Average overall exposure: {}", average_exposure);
    println!("Total gamma exposure: {}", total_exposure);
    println!("----------------------------------------------------");

    Ok(())
}
