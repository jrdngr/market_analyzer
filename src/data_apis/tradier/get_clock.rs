use std::convert::{TryFrom, TryInto};

use crate::types;
use serde::{Deserialize, Serialize};

pub async fn get_clock() -> anyhow::Result<types::Clock> {
    let access_token = std::env::var(super::ACCESS_TOKEN_ENV)?;
    let url = format!("{}/markets/clock", super::BASE_URL);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .text()
        .await?;

    let clock: ClockResponse = serde_json::from_str(&body)?;

    Ok(clock.clock.try_into()?)
}

#[derive(Clone, Debug, Deserialize)]
struct ClockResponse {
    clock: ClockResponseInner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClockResponseInner {
    pub timestamp: u64,
    pub date: String,
    pub description: String,
    pub state: String,
    pub next_change: String,
    pub next_state: String,
}

impl TryFrom<ClockResponseInner> for types::Clock {
    type Error = anyhow::Error;

    fn try_from(clock: ClockResponseInner) -> Result<Self, Self::Error> {
        let mut split_change = clock.next_change.split(':');
        let next_change_hours: i64 = split_change
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid next_change: {}", clock.next_change))?
            .parse()?;
        let next_change_minutes: i64 = split_change
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid next_change: {}", clock.next_change))?
            .parse()?;

        Ok(Self {
            timestamp: clock.timestamp,
            date: clock.date,
            description: clock.description,
            state: clock.state.parse()?,
            next_change_minutes: next_change_hours * 60 + next_change_minutes,
            next_state: clock.next_state.parse()?,
        })
    }
}
