pub mod file;

use crate::{data_apis, types::OptionInfo};
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
        let mut option_chain_interval = tokio::time::interval(Duration::from_secs(60 * 60));
        let mut symbol_delay = tokio::time::interval(Duration::from_secs(30));
        loop {
            let symbols = {
                let db = db.lock().await;
                db.symbols()
            };

            for symbol in symbols {
                if let Err(e) = update_symbol(&symbol, db.clone()).await {
                    log::error!("{}", e);
                }
                symbol_delay.tick().await;
            }
            option_chain_interval.tick().await;
        }
    });

    Ok(())
}

pub async fn update_symbol(symbol: &str, db: Arc<Mutex<FileDb>>) -> anyhow::Result<()> {
    use data_apis::tradier;

    log::info!("Updating data for {}", symbol);
    let option_chain = tradier::get_option_chain(&symbol.to_uppercase()).await?;
    let mut db = db.lock().await;
    db.add_option_info(&symbol, option_chain)?;
    log::info!("Successfully updated data for {}", symbol);

    Ok(())
}
