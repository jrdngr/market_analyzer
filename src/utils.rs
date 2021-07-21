use std::convert::TryFrom;

use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum FloatOrString {
    Float(f64),
    String(String),
}

impl TryFrom<FloatOrString> for f64 {
    type Error = String;

    fn try_from(n: FloatOrString) -> Result<Self, Self::Error> {
        match n {
            FloatOrString::Float(f) => Ok(f),
            FloatOrString::String(s) => {
                if s == "NaN" {
                    Ok(f64::NAN)
                } else {
                    Err(s)
                }
            }
        }
    }
}

pub fn deserialize_f64_with_nan<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: FloatOrString = Deserialize::deserialize(deserializer)?;
    match f64::try_from(value) {
        Ok(value) => Ok(value),
        Err(string) => Err(serde::de::Error::custom(format!(
            "Invalid string: {}",
            string
        ))),
    }
}
