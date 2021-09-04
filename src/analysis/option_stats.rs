use crate::types::{
    stats::{HedgeExposure, StrikeStats},
    OptionInfo, OptionType,
};
use std::collections::HashMap;

pub fn option_stats(option_chain: &[OptionInfo]) -> Vec<StrikeStats> {
    let mut strike_to_stats: HashMap<String, StrikeStats> = HashMap::new();

    for option in option_chain {
        let strike = option.strike.to_string();
        let gamma = option.open_interest as f64 * option.gamma();
        let vanna = option.open_interest as f64 * option.vanna();
        let charm = option.open_interest as f64 * option.charm();

        let gamma = if gamma.is_finite() { gamma } else { 0.0 };
        let vanna = if vanna.is_finite() { vanna } else { 0.0 };
        let charm = if charm.is_finite() { charm } else { 0.0 };

        if let Some(aggregate_stats) = strike_to_stats.get_mut(&strike) {
            aggregate_stats.open_interest += option.open_interest;

            if option.option_type == OptionType::Call {
                aggregate_stats.call_exposure.gamma += gamma;
                aggregate_stats.call_exposure.vanna += vanna;
                aggregate_stats.call_exposure.charm += charm;
            } else {
                aggregate_stats.put_exposure.gamma += gamma;
                aggregate_stats.put_exposure.vanna += vanna;
                aggregate_stats.put_exposure.charm += charm;
            }
        } else {
            let stats = StrikeStats {
                strike: option.strike,
                open_interest: option.open_interest,
                call_exposure: HedgeExposure {
                    gamma,
                    vanna,
                    charm,
                },
                put_exposure: HedgeExposure {
                    gamma,
                    vanna,
                    charm,
                },
            };

            strike_to_stats.insert(strike, stats);
        }
    }

    strike_to_stats.into_iter().map(|(_, v)| v).collect()
}
