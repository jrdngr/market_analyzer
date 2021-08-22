pub mod file;

use crate::{data_apis::tradier, types::OptionInfo};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

pub use file::FileDb;

pub async fn option_chain(symbol: &str, db: Arc<Mutex<FileDb>>) -> anyhow::Result<Vec<OptionInfo>> {
    let has_symbol = {
        let db = db.lock().await;
        db.has_symbol(symbol)
    };

    if !has_symbol {
        update_symbol(symbol, db.clone()).await?;
    }

    let db = db.lock().await;
    let option_chain = db
        .option_chain(symbol)
        .ok_or_else(|| anyhow::anyhow!("Error loading data for {}", symbol))?;

    Ok(option_chain.clone())
}

pub fn start_db_update_loop(db: Arc<Mutex<FileDb>>) -> anyhow::Result<()> {
    tokio::task::spawn(async move {
        let mut symbol_delay = tokio::time::interval(Duration::from_secs(60));
        loop {
            log::info!("Updating all symbols");

            let symbols = {
                let db = db.lock().await;
                db.symbols()
            };

            for symbol in symbols {
                symbol_delay.tick().await;
                if let Err(e) = update_symbol(&symbol, db.clone()).await {
                    log::error!("{}", e);
                }
            }

            log::info!("Successfully updated all symbols");

            let sleep_duration = duration_until_next_check().await;
            log::info!("Next update {} minutes", sleep_duration.as_secs() / 60);

            tokio::time::sleep(sleep_duration).await;
        }
    });

    Ok(())
}

pub async fn duration_until_next_check() -> Duration {
    let clock = match tradier::get_clock().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("{}", e);
            return Duration::from_secs(60 * 60);
        }
    };
    let next_check_minutes = 0.max(clock.next_change_minutes - 60) as u64;
    let next_check_seconds = next_check_minutes * 60;
    Duration::from_secs(next_check_seconds)
}

pub async fn update_symbol(symbol: &str, db: Arc<Mutex<FileDb>>) -> anyhow::Result<()> {
    log::info!("Updating data for {}", symbol);
    let option_chain = tradier::get_option_chain(&symbol.to_uppercase()).await?;
    let mut db = db.lock().await;
    db.add_option_info(&symbol, option_chain)?;
    log::info!("Successfully updated data for {}", symbol);

    Ok(())
}
