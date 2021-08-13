use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

use crate::types::{GammaExposureStats, OptionInfo};

pub const FILE_NAME: &str = "db.gz";

pub type Symbol = String;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileDb {
    gex: HashMap<Symbol, Vec<GammaExposureStats>>,
    options: HashMap<Symbol, Vec<OptionInfo>>,
}

impl FileDb {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_data(
        gex: HashMap<Symbol, Vec<GammaExposureStats>>,
        options: HashMap<Symbol, Vec<OptionInfo>>,
    ) -> Self {
        Self { gex, options }
    }

    pub fn from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;

        let mut decoder = GzDecoder::new(&*bytes);
        let mut decoded_bytes = Vec::new();
        decoder.read_exact(&mut decoded_bytes)?;

        Ok(serde_json::from_slice(&decoded_bytes)?)
    }

    pub fn load() -> anyhow::Result<Self> {
        Self::from_file(FILE_NAME)
    }

    pub fn add_gamma_exposure_stats(&mut self, data: GammaExposureStats) -> anyhow::Result<()> {
        let entry = self.gex.entry(data.symbol.clone()).or_insert_with(Vec::new);
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

    pub fn current_gamma_exposure_stats(&self, symbol: &str) -> Option<&GammaExposureStats> {
        self.gex.get(symbol).map(|v| v.last()).flatten()
    }

    pub fn current_option_info(&self, symbol: &str) -> Option<&OptionInfo> {
        self.options.get(symbol).map(|v| v.last()).flatten()
    }

    fn write(&self) -> anyhow::Result<()> {
        let json = serde_json::to_vec_pretty(&self)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&json)?;
        let compressed_bytes = encoder.finish()?;

        std::fs::write(FILE_NAME, &compressed_bytes)?;

        Ok(())
    }
}
