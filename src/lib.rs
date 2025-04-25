use anyhow::{anyhow, bail, Result};
use log::{error, warn};
use std::{fs::File, io::Read, path::Path};

mod bin;
mod models;
pub mod version;
pub use models::*;

use bin::BinData;

#[derive(Debug, Clone)]
pub struct Solution {
    pub name: String,
    pub puzzle_id: i32,
    pub parts: Vec<Part>,
}

impl Solution {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            error!("File {} does not exist", path.display());
            bail!("File not found");
        }
        let mut reader = File::open(path).map_err(|_| {
            warn!("Failed to open file {}", path.display());
            anyhow!("Failed to open file")
        })?;
        let res = Solution::read(&mut reader);
        if reader.read_to_end(&mut Vec::new())? > 0 {
            warn!("File {} is not empty after reading", path.display());
            bail!("File not empty")
        } else {
            res.map_err(|s| anyhow!("{s}"))
        }
    }
}
