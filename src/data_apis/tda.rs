mod get_ohlc;
mod get_option_chain;
mod get_quote;

pub use get_ohlc::{_get_ohlc_authenticated, get_ohlc};
pub use get_option_chain::{_get_option_chain_authenticated, get_option_chain};
pub use get_quote::{get_quote, get_quote_authenticated};

const API_KEY_ENV: &str = "API_KEY";
const BASE_URL: &str = "https://api.tdameritrade.com/v1";
