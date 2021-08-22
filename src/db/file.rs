use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

use crate::types::{GammaExposureStats, OptionInfo};

pub const FILE_NAME: &str = "data/db.gz";

pub type Symbol = String;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileDb {
    gex: HashMap<Symbol, Vec<GammaExposureStats>>,
    gex_aggregate: HashMap<Symbol, Vec<GammaExposureStats>>,
    options: HashMap<Symbol, Vec<OptionInfo>>,
}

impl FileDb {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_data(
        gex: HashMap<Symbol, Vec<GammaExposureStats>>,
        gex_aggregate: HashMap<Symbol, Vec<GammaExposureStats>>,
        options: HashMap<Symbol, Vec<OptionInfo>>,
    ) -> Self {
        Self {
            gex,
            gex_aggregate,
            options,
        }
    }

    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;

        let mut decoder = GzDecoder::new(&*bytes);
        let mut decoded_bytes = String::new();
        decoder.read_to_string(&mut decoded_bytes)?;

        Ok(serde_json::from_str(&decoded_bytes)?)
    }

    pub fn load() -> anyhow::Result<Self> {
        Self::from_file(FILE_NAME)
    }

    pub fn add_gamma_exposure(&mut self, data: GammaExposureStats) -> anyhow::Result<()> {
        let entry = self.gex.entry(data.symbol.clone()).or_insert_with(Vec::new);
        entry.push(data);
        self.write()?;

        Ok(())
    }

    pub fn add_gamma_exposure_aggregate(&mut self, data: GammaExposureStats) -> anyhow::Result<()> {
        let entry = self
            .gex_aggregate
            .entry(data.symbol.clone())
            .or_insert_with(Vec::new);
        entry.push(data);
        self.write()?;

        Ok(())
    }

    pub fn add_option_info(&mut self, data: OptionInfo) -> anyhow::Result<()> {
        let entry = self
            .options
            .entry(data.symbol.clone())
            .or_insert_with(Vec::new);
        entry.push(data);
        self.write()?;

        Ok(())
    }

    pub fn current_gamma_exposure(&self, symbol: &str) -> Option<&GammaExposureStats> {
        self.gex.get(symbol).map(|v| v.last()).flatten()
    }

    pub fn current_gamma_exposure_aggregate(&self, symbol: &str) -> Option<&GammaExposureStats> {
        self.gex_aggregate.get(symbol).map(|v| v.last()).flatten()
    }

    pub fn current_option_info(&self, symbol: &str) -> Option<&OptionInfo> {
        self.options.get(symbol).map(|v| v.last()).flatten()
    }

    pub fn symbols(&self) -> Vec<String> {
        self.options.keys().cloned().collect()
    }

    fn write(&self) -> anyhow::Result<()> {
        let json = serde_json::to_vec_pretty(&self)?;

        // Debug
        // std::fs::write("data/db.json", &json)?;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&json)?;
        let compressed_bytes = encoder.finish()?;

        std::fs::write(FILE_NAME, &compressed_bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db() {
        let mut db = FileDb::new();

        db.add_gamma_exposure(GammaExposureStats::test()).unwrap();
        db.add_gamma_exposure(GammaExposureStats::test()).unwrap();
        db.add_gamma_exposure(GammaExposureStats::test()).unwrap();

        db.add_option_info(OptionInfo::test()).unwrap();
        db.add_option_info(OptionInfo::test()).unwrap();
        db.add_option_info(OptionInfo::test()).unwrap();

        let gex = db.current_gamma_exposure("TST").unwrap();
        assert_eq!(gex.symbol, "TST");

        let oi = db.current_option_info("TST").unwrap();
        assert_eq!(oi.symbol, "TST");

        db.write().unwrap();

        let db2 = FileDb::load().unwrap();

        assert_eq!(db2.current_gamma_exposure("TST").unwrap().symbol, "TST");
        assert_eq!(db2.current_option_info("TST").unwrap().symbol, "TST");
    }
}
