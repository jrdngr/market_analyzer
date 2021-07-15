use std::{collections::BTreeMap, path::Path};

use chrono::Utc;
use rust_decimal::Decimal;

use crate::td;

const REPORT_PATH: &str = "reports";

pub async fn gamma_exposure_by_price(symbol: &str, force_download: bool) -> anyhow::Result<BTreeMap<Decimal, f64>> {
    dotenv::dotenv()?;

    let options = td::get_option_chain(symbol, force_download).await?;

    let mut price_gamma_exposure: BTreeMap<Decimal, f64> = BTreeMap::new();

    for contracts in options.call_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest;
                match price_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        price_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }
    for contracts in options.put_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest * -1.0;
                match price_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        price_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }

    Ok(price_gamma_exposure)
}

pub async fn print_gamma_exposure_by_price(symbol: &str, force_download: bool) -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let options = td::get_option_chain(symbol, force_download).await?;

    let mut call_exposure = 0.0;
    let mut call_count = 0;
    let mut put_exposure = 0.0;
    let mut put_count = 0;
    let mut price_gamma_exposure: BTreeMap<Decimal, f64> = BTreeMap::new();

    for contracts in options.call_exp_date_map.values() {
        for (strike, options) in contracts {
            for option in options {
                let exposure = option.gamma * option.open_interest;
                call_exposure += exposure;
                call_count += 1;
                match price_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        price_gamma_exposure.insert(*strike, exposure);
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
                match price_gamma_exposure.get_mut(strike) {
                    Some(exp) => *exp += exposure,
                    None => {
                        price_gamma_exposure.insert(*strike, exposure);
                    }
                }
            }
        }
    }

    let total_exposure = call_exposure + put_exposure;
    let average_call_exposure = call_exposure / (call_count as f64);
    let average_put_exposure = put_exposure / (put_count as f64);
    let average_exposure =
        (call_exposure.abs() + put_exposure.abs()) / (price_gamma_exposure.len() as f64);

    println!("----------------------------------------------------");
    let mut csv = String::new();
    for (strike, exposure) in price_gamma_exposure {
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

