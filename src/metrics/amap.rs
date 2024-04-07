use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for AmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl AmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}
