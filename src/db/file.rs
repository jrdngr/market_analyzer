use std::{
    collections::HashMap,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};

use crate::types::OptionInfo;

pub const DEFAULT_FILE_PATH: &str = "data/db.gz";

pub type Symbol = String;
pub type OptionSnapshot = Vec<OptionInfo>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileDb {
    file_path: PathBuf,
    options: HashMap<Symbol, Vec<OptionSnapshot>>,
}

impl FileDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self::from_data(path, HashMap::new())
    }

    pub fn from_data(
        path: impl AsRef<Path>,
        options: HashMap<Symbol, Vec<OptionSnapshot>>,
    ) -> Self {
        Self {
            file_path: path.as_ref().into(),
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
        Self::from_file(DEFAULT_FILE_PATH)
    }

    pub fn add_option_info(&mut self, symbol: &str, data: Vec<OptionInfo>) -> anyhow::Result<()> {
        let entry = self
            .options
            .entry(symbol.to_string())
            .or_insert_with(Vec::new);

        entry.push(data);
        self.write()?;

        Ok(())
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.options.contains_key(symbol)
    }

    pub fn option_chain(&self, symbol: &str) -> Option<&Vec<OptionInfo>> {
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

        std::fs::write(&self.file_path, &compressed_bytes)?;

        Ok(())
    }
}

impl Default for FileDb {
    fn default() -> Self {
        Self::new(DEFAULT_FILE_PATH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_FILE_PATH: &str = "data/test_db.gz";

    #[test]
    fn db() {
        let mut db = FileDb::new(TEST_FILE_PATH);

        db.add_option_info("TST", vec![OptionInfo::test()]).unwrap();
        db.add_option_info("TST", vec![OptionInfo::test()]).unwrap();
        db.add_option_info("TST", vec![OptionInfo::test()]).unwrap();

        let oi = db.option_chain("TST").unwrap();
        assert_eq!(oi[0].symbol, "TST");

        db.write().unwrap();

        let db2 = FileDb::from_file(TEST_FILE_PATH).unwrap();

        assert_eq!(db2.option_chain("TST").unwrap()[0].symbol, "TST");
    }
}
