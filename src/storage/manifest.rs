use atomic_write_file::AtomicWriteFile;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::constants::{LOG_FILENAME, MANIFEST_FILENAME};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub entry_count: u64,
    pub last_checksum: u32,
    #[serde(default)]
    pub oldest_timestamp: Option<i64>,
    #[serde(default)]
    pub newest_timestamp: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for Manifest {
    fn default() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        Self {
            version: 1,
            entry_count: 0,
            last_checksum: 0,
            oldest_timestamp: None,
            newest_timestamp: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Manifest {
    pub fn path(base_path: &Path) -> PathBuf {
        base_path.join(MANIFEST_FILENAME)
    }

    pub fn log_path(base_path: &Path) -> PathBuf {
        base_path.join(LOG_FILENAME)
    }

    pub fn load(base_path: &Path) -> Result<Self, crate::Error> {
        let path = Self::path(base_path);
        if !path.exists() {
            return Err(crate::Error::ManifestNotFound);
        }

        let content = std::fs::read_to_string(&path)?;

        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, base_path: &Path) -> Result<(), crate::Error> {
        let path = Self::path(base_path);
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::Error::Serialization(e.to_string()))?;

        let mut file = AtomicWriteFile::open(&path)?;
        file.write_all(content.as_bytes())?;
        file.commit()?;

        Ok(())
    }
}
