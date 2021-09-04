use crate::math::{standard_normal_cdf, standard_normal_probability_density};

const R: f64 = 0.0;

pub fn call_price(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    use std::f64::consts::E;

    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let d2 = d2(d1, sigma, expiration_time, current_time);
    let t = expiration_time - current_time;

    (standard_normal_cdf(d1) * current_price) - (standard_normal_cdf(d2) * strike * E.powf(-R * t))
}

pub fn put_price(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    use std::f64::consts::E;

    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let d2 = d2(d1, sigma, expiration_time, current_time);
    let t = expiration_time - current_time;

    (standard_normal_cdf(-d2) * strike * E.powf(-R * t))
        - (standard_normal_cdf(-d1) * current_price)
}

pub fn call_delta(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    standard_normal_cdf(d1(
        sigma,
        expiration_time,
        current_time,
        current_price,
        strike,
    ))
}

pub fn put_delta(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    standard_normal_cdf(d1(
        sigma,
        expiration_time,
        current_time,
        current_price,
        strike,
    )) - 1.0
}

pub fn gamma(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let t = expiration_time - current_time;
    standard_normal_probability_density(d1) / (current_price * sigma * t.sqrt())
}

pub fn theta(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    use std::f64::consts::E;

    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let d2 = d2(d1, sigma, expiration_time, current_time);
    let t = expiration_time - current_time;

    let a = (current_price * standard_normal_probability_density(d1) * sigma) / (2.0 * t.sqrt());
    let b = R * strike * E.powf(-R * t) * standard_normal_cdf(d2);

    (-a - b) / 365.0
}

pub fn vega(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let t = expiration_time - current_time;

    (current_price * standard_normal_probability_density(d1) * t.sqrt()) / 100.0
}

pub fn vanna(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let d2 = d2(d1, sigma, expiration_time, current_time);
    (-1.0 * standard_normal_probability_density(d1) * (d2 / sigma)) / 100.0
}

pub fn charm(
    sigma: f64,
    expiration_time: f64,
    current_time: f64,
    current_price: f64,
    strike: f64,
) -> f64 {
    let t = expiration_time - current_time;
    let d1 = d1(sigma, expiration_time, current_time, current_price, strike);
    let d2 = d2(d1, sigma, expiration_time, current_time);

    let numerator = (2.0 * R * t) - (d2 * sigma * t.sqrt());
    let denominator = 2.0 * t * sigma * t.sqrt();
    -standard_normal_probability_density(d1) * (numerator / denominator)
}

fn d1(sigma: f64, expiration_time: f64, current_time: f64, current_price: f64, strike: f64) -> f64 {
    let t = expiration_time - current_time;
    ((current_price / strike).ln() + (R + (sigma.powi(2) / 2.0)) * t) / (sigma * t.sqrt())
}

fn d2(d1: f64, sigma: f64, expiration_time: f64, current_time: f64) -> f64 {
    let t = expiration_time - current_time;
    d1 - (sigma * t.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOAT_ERROR: f64 = 0.0001;
    const EXPIRATION: f64 = 180.0 / 365.0;

    #[test]
    fn test_call_price() {
        assert_float_eq(1.8824, call_price(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_put_price() {
        assert_float_eq(0.8824, put_price(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_call_delta() {
        assert_float_eq(0.6828, call_delta(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_put_delta() {
        assert_float_eq(-0.3172, put_delta(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_gamma() {
        assert_float_eq(0.1015, gamma(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_theta() {
        assert_float_eq(-0.0035, theta(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_vega() {
        assert_float_eq(0.0250, vega(0.5, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_vanna() {
        assert_float_eq(-0.01798, vanna(0.1, EXPIRATION, 0.0, 10.0, 9.0));
        assert_float_eq(-0.0008871, vanna(0.5, EXPIRATION, 0.0, 10.0, 9.0));
        assert_float_eq(0.0007075, vanna(1.0, EXPIRATION, 0.0, 10.0, 9.0));
    }

    #[test]
    fn test_charm() {
        assert_float_eq(0.1823, charm(0.1, EXPIRATION, 0.0, 10.0, 9.0));
        assert_float_eq(0.0449, charm(0.5, EXPIRATION, 0.0, 10.0, 9.0));
        assert_float_eq(-0.0717, charm(1.0, EXPIRATION, 0.0, 10.0, 9.0));
    }

    fn assert_float_eq(actual: f64, expected: f64) {
        let diff = actual - expected;
        assert!(diff.abs() < FLOAT_ERROR, "{} != {}", actual, expected);
    }
}
