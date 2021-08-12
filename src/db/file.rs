use std::collections::HashMap;

use crate::types::{GammaExposureStats, OptionInfo};

pub const FILE_NAME: &str = "db.gz";

pub struct FileDb {
    pub gex: HashMap<String, GammaExposureStats>,
    pub options: HashMap<String, OptionInfo>,
}
