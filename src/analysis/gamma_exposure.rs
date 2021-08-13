use std::collections::BTreeMap;

use chrono::{Date, FixedOffset, Local, TimeZone};

use crate::{math::bs::gamma, types::{GammaExposureOptions, OptionInfo, OptionType, gex::{GammaExposure, GammaExposureStats}}};

impl GammaExposureStats {
    pub fn new(
        symbol: impl Into<String>,
        strike_to_gamma_exposure: &BTreeMap<String, f64>,
    ) -> anyhow::Result<Self> {
        let mut positive_sum: f64 = 0.0;
        let mut positive_count = 0;
        let mut negative_sum: f64 = 0.0;
        let mut negative_count = 0;
        let mut maximum_gamma_exposure: f64 = 0.0;
        let mut minimum_gamma_exposure: f64 = 0.0;
        let mut absolute_maximum: f64 = 0.0;
        let mut absolute_minimum: f64 = f64::MAX;
        let mut weighted_positive_sum: f64 = 0.0;
        let mut weighted_negative_sum: f64 = 0.0;
        let mut absolute_maximum_price: f64 = 0.0;
        let mut absolute_minimum_price: f64 = f64::MAX;

        for (strike, exposure) in strike_to_gamma_exposure {
            let strike: f64 = strike.parse()?;
            if *exposure >= 0.0 {
                positive_sum += exposure;
                weighted_positive_sum += strike * exposure;
                positive_count += 1;
            } else {
                negative_sum += exposure;
                weighted_negative_sum += strike * *exposure;
                negative_count += 1;
            }
            maximum_gamma_exposure = maximum_gamma_exposure.max(*exposure);
            minimum_gamma_exposure = minimum_gamma_exposure.min(*exposure);

            if exposure.abs() >= absolute_maximum {
                absolute_maximum = exposure.abs();
                absolute_maximum_price = strike;
            }

            if exposure.abs() <= absolute_minimum {
                absolute_minimum = exposure.abs();
                absolute_minimum_price = strike;
            }
        }

        positive_count = positive_count.max(1);
        negative_count = negative_count.max(1);

        let average_positive_exposure = positive_sum / positive_count as f64;
        let average_negative_exposure = negative_sum / negative_count as f64;
        let average_absolute_exposure =
            (positive_sum.abs() + negative_sum.abs()) / (positive_count + negative_count) as f64;

        let weighted_average_absolute_price = (weighted_positive_sum.abs()
            + weighted_negative_sum.abs())
            / (positive_sum.abs() + negative_sum.abs());
        let weighted_average_positive_price = weighted_positive_sum / positive_sum;
        let weighted_average_negative_price = weighted_negative_sum / negative_sum;

        let mut prices: Vec<GammaExposure> = strike_to_gamma_exposure
            .iter()
            .map(|(strike, exposure)| GammaExposure::new(strike.clone(), *exposure))
            .collect();

        prices.sort_by(|p1, p2| p1.strike.cmp(&p2.strike));

        Ok(Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            symbol: symbol.into(),
            prices,
            average_absolute_exposure,
            average_positive_exposure,
            average_negative_exposure,
            maximum_gamma_exposure,
            minimum_gamma_exposure,
            absolute_maximum,
            absolute_minimum,
            weighted_average_absolute_price,
            weighted_average_positive_price,
            weighted_average_negative_price,
            absolute_maximum_price,
            absolute_minimum_price,
        })
    }
}

impl GammaExposure {
    pub fn new(strike: String, gamma_exposure: f64) -> Self {
        Self {
            strike,
            gamma_exposure,
        }
    }
}

pub fn gamma_exposure_by_price(
    symbol: &str,
    option_chain: Vec<OptionInfo>,
    options: GammaExposureOptions,
) -> anyhow::Result<BTreeMap<String, f64>> {
    let mut strike_to_gamma_exposure: BTreeMap<String, f64> = BTreeMap::new();

    for option in option_chain {
        if let Some(min) = options.min_strike {
            if option.strike < min {
                continue;
            }
        }

        if let Some(max) = options.max_strike {
            if option.strike > max {
                continue;
            }
        }

        let strike = option.strike.to_string();
        if let Some(gamma) = option.gamma {
            let mut exposure = if gamma > 1.0 || gamma < -1.0 {
                0.0
            } else {
                gamma * option.open_interest as f64
            };
            if option.option_type == OptionType::Put {
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

    Ok(strike_to_gamma_exposure)
}

pub fn gamma_exposure(
    symbol: &str,
    option_chain: Vec<OptionInfo>,
    options: GammaExposureOptions,
) -> anyhow::Result<GammaExposureStats> {
    if options.aggregate {
        gamma_exposure_aggregate(symbol, option_chain, options)
    } else {
        gamma_exposure_stats(symbol, option_chain, options)
    }
}

pub fn gamma_exposure_stats(
    symbol: &str,
    option_chain: Vec<OptionInfo>,
    options: GammaExposureOptions,
) -> anyhow::Result<GammaExposureStats> {
    let strike_to_gamma_exposure = gamma_exposure_by_price(symbol, option_chain, options)?;
    Ok(GammaExposureStats::new(symbol, &strike_to_gamma_exposure)?)
}

pub fn gamma_exposure_aggregate(
    symbol: &str,
    option_chain: Vec<OptionInfo>,
    options: GammaExposureOptions,
) -> anyhow::Result<GammaExposureStats> {
    let now = Local::now().date();
    let mut strike_to_gamma_exposure_aggregate: BTreeMap<String, f64> = BTreeMap::new();

    let min_price = options.min_strike.unwrap_or_else(|| {
        option_chain
            .iter()
            .map(|o| o.strike)
            .min_by(|s1, s2| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Less))
            .unwrap_or(0.0)
    });

    let max_price = options.max_strike.unwrap_or_else(|| {
        option_chain
            .iter()
            .map(|o| o.strike)
            .max_by(|s1, s2| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Less))
            .unwrap_or(0.0)
    });

    let price_offset = 0.5;

    for option in option_chain {
        let expiration_date = parse_date(&option.expiration_date)?;
        let days_remaining = expiration_date.signed_duration_since(now).num_days();

        let sigma = option.mid_iv.unwrap_or(0.0);
        let expiration_time = days_remaining as f64 / 365.0;
        let current_time = 0.0;
        let strike = option.strike;

        let mut price = min_price.floor();
        while price <= max_price {
            let price_string = price.to_string();
            let gamma = gamma(sigma, expiration_time, current_time, price, strike);

            let mut exposure = if !(-1.0..=1.0).contains(&gamma) || gamma.is_nan() {
                0.0
            } else {
                gamma * option.open_interest as f64
            };
            if option.option_type == OptionType::Put {
                exposure *= -1.0;
            }
            match strike_to_gamma_exposure_aggregate.get_mut(&price_string) {
                Some(exp) => *exp += exposure,
                None => {
                    strike_to_gamma_exposure_aggregate.insert(price_string.clone(), exposure);
                }
            }

            price += price_offset;
        }
    }

    // dbg!(&strike_to_gamma_exposure_aggregate);

    Ok(GammaExposureStats::new(
        symbol,
        &strike_to_gamma_exposure_aggregate,
    )?)
}

fn parse_date(date: &str) -> anyhow::Result<Date<FixedOffset>> {
    let mut split_date = date.split('-');

    let y = split_date
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid year"))?
        .parse()?;
    let m = split_date
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid month"))?
        .parse()?;
    let d = split_date
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid day"))?
        .parse()?;

    let ny_offset = FixedOffset::east(5 * 3600);
    Ok(ny_offset.ymd(y, m, d))
}
