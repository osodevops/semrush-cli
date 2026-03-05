use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tracing::debug;

pub struct DiskCache {
    dir: PathBuf,
    ttl: Duration,
}

impl DiskCache {
    pub fn new(dir: PathBuf, ttl_secs: u64) -> Self {
        Self {
            dir,
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn cache_key(&self, endpoint: &str, params: &str) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(endpoint.as_bytes());
        hasher.update(b"|");
        hasher.update(params.as_bytes());
        let hash = hex::encode(hasher.finalize());
        self.dir.join(format!("{hash}.json"))
    }

    pub fn get(&self, endpoint: &str, params: &str) -> Option<String> {
        let path = self.cache_key(endpoint, params);
        if !path.exists() {
            return None;
        }

        let metadata = std::fs::metadata(&path).ok()?;
        let modified = metadata.modified().ok()?;
        let age = SystemTime::now().duration_since(modified).ok()?;

        if age > self.ttl {
            debug!(path = %path.display(), "Cache entry expired");
            let _ = std::fs::remove_file(&path);
            return None;
        }

        debug!(path = %path.display(), "Cache hit");
        std::fs::read_to_string(&path).ok()
    }

    pub fn set(&self, endpoint: &str, params: &str, data: &str) {
        let path = self.cache_key(endpoint, params);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Err(e) = std::fs::write(&path, data) {
            debug!(error = %e, "Failed to write cache entry");
        }
    }

    pub fn clear(&self) -> std::io::Result<u64> {
        let mut count = 0;
        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)? {
                let entry = entry?;
                if entry.path().extension().is_some_and(|ext| ext == "json") {
                    std::fs::remove_file(entry.path())?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    pub fn stats(&self) -> (u64, u64) {
        let mut count = 0u64;
        let mut size = 0u64;
        if let Ok(entries) = std::fs::read_dir(&self.dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "json") {
                    count += 1;
                    size += entry.metadata().map(|m| m.len()).unwrap_or(0);
                }
            }
        }
        (count, size)
    }
}
