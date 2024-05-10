use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // 使用 DashMap 就不需要先获取锁了
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

// 优化输出格式
impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
