pub mod bs;

use statrs::distribution::{ContinuousCDF, Normal};

pub fn standard_normal_cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    n.cdf(x)
}

pub fn standard_normal_probability_density(x: f64) -> f64 {
    use std::f64::consts::{E, PI};

    E.powf(-(x).powi(2) / 2.0) / (2.0 * PI).sqrt()
}
