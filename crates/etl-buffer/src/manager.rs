use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::warn;

use etl_config::BufferSettings;
use etl_proto::common::Record;

#[derive(Debug, Clone)]
pub struct BufferedRecord {
    pub record: Record,
    pub source_id: String,
    pub pipeline_id: String,
    pub target_stage_id: String,
    pub buffered_at: Instant,
    pub retry_count: u32,
}

struct StageBuffer {
    records: VecDeque<BufferedRecord>,
    max_size: usize,
}

impl StageBuffer {
    fn new(max_size: usize) -> Self {
        Self {
            records: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, record: BufferedRecord) -> bool {
        if self.records.len() >= self.max_size {
            return false;
        }
        self.records.push_back(record);
        true
    }

    fn pop_batch(&mut self, max_batch_size: usize) -> Vec<BufferedRecord> {
        let count = std::cmp::min(max_batch_size, self.records.len());
        self.records.drain(..count).collect()
    }

    fn len(&self) -> usize {
        self.records.len()
    }

    fn is_full(&self) -> bool {
        self.records.len() >= self.max_size
    }

    fn utilization(&self) -> f64 {
        self.records.len() as f64 / self.max_size as f64
    }
}

pub struct BufferManager {
    stage_buffers: Arc<RwLock<HashMap<String, StageBuffer>>>,
    source_buffers: Arc<RwLock<HashMap<String, VecDeque<BufferedRecord>>>>,

    max_total_records: usize,
    max_per_stage: usize,
    max_per_source: usize,
    backpressure_threshold: f64,

    total_records: Arc<RwLock<usize>>,
}

impl BufferManager {
    pub fn new(settings: BufferSettings) -> Self {
        Self {
            stage_buffers: Arc::new(RwLock::new(HashMap::new())),
            source_buffers: Arc::new(RwLock::new(HashMap::new())),
            max_total_records: settings.max_total_records,
            max_per_stage: settings.max_per_stage,
            max_per_source: settings.max_per_source,
            backpressure_threshold: settings.backpressure_threshold,
            total_records: Arc::new(RwLock::new(0)),
        }
    }

    pub fn with_limits(
        max_total_records: usize,
        max_per_stage: usize,
        max_per_source: usize,
        backpressure_threshold: f64,
    ) -> Self {
        Self {
            stage_buffers: Arc::new(RwLock::new(HashMap::new())),
            source_buffers: Arc::new(RwLock::new(HashMap::new())),
            max_total_records,
            max_per_stage,
            max_per_source,
            backpressure_threshold,
            total_records: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn buffer_for_stage(
        &self,
        stage_id: &str,
        record: BufferedRecord,
    ) -> Result<()> {
        let mut total = self.total_records.write().await;
        if *total >= self.max_total_records {
            return Err(anyhow::anyhow!("Global buffer full"));
        }

        let mut buffers = self.stage_buffers.write().await;
        let buffer = buffers
            .entry(stage_id.to_string())
            .or_insert_with(|| StageBuffer::new(self.max_per_stage));

        if buffer.push(record) {
            *total += 1;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Stage buffer full: {}", stage_id))
        }
    }

    pub async fn buffer_batch_for_stage(
        &self,
        stage_id: &str,
        records: Vec<BufferedRecord>,
    ) -> Result<usize> {
        let mut total = self.total_records.write().await;
        let available = self.max_total_records.saturating_sub(*total);

        if available == 0 {
            return Err(anyhow::anyhow!("Global buffer full"));
        }

        let mut buffers = self.stage_buffers.write().await;
        let buffer = buffers
            .entry(stage_id.to_string())
            .or_insert_with(|| StageBuffer::new(self.max_per_stage));

        let mut buffered = 0;
        for record in records {
            if buffered >= available {
                break;
            }
            if buffer.push(record) {
                buffered += 1;
            } else {
                break;
            }
        }

        *total += buffered;
        Ok(buffered)
    }

    pub async fn get_batch(&self, stage_id: &str, max_batch_size: usize) -> Vec<BufferedRecord> {
        let mut buffers = self.stage_buffers.write().await;
        let mut total = self.total_records.write().await;

        if let Some(buffer) = buffers.get_mut(stage_id) {
            let batch = buffer.pop_batch(max_batch_size);
            *total = total.saturating_sub(batch.len());
            batch
        } else {
            Vec::new()
        }
    }

    pub async fn return_to_buffer(&self, stage_id: &str, records: Vec<BufferedRecord>) {
        let mut buffers = self.stage_buffers.write().await;
        let mut total = self.total_records.write().await;

        let buffer = buffers
            .entry(stage_id.to_string())
            .or_insert_with(|| StageBuffer::new(self.max_per_stage));

        for mut record in records {
            record.retry_count += 1;
            if buffer.push(record) {
                *total += 1;
            } else {
                warn!(stage_id = %stage_id, "Failed to return record to buffer - buffer full");
            }
        }
    }

    pub async fn should_backpressure(&self, source_id: &str) -> bool {
        let total = *self.total_records.read().await;
        let global_utilization = total as f64 / self.max_total_records as f64;

        if global_utilization > self.backpressure_threshold {
            return true;
        }

        let source_buffers = self.source_buffers.read().await;
        if let Some(buffer) = source_buffers.get(source_id) {
            let source_utilization = buffer.len() as f64 / self.max_per_source as f64;
            if source_utilization > self.backpressure_threshold {
                return true;
            }
        }

        false
    }

    pub async fn available_credits(&self, source_id: &str) -> u64 {
        let total = *self.total_records.read().await;
        let global_available = self.max_total_records.saturating_sub(total);

        let source_buffers = self.source_buffers.read().await;
        let source_used = source_buffers
            .get(source_id)
            .map(|b| b.len())
            .unwrap_or(0);
        let source_available = self.max_per_source.saturating_sub(source_used);

        std::cmp::min(global_available, source_available) as u64
    }

    pub async fn get_stage_buffer_size(&self, stage_id: &str) -> usize {
        let buffers = self.stage_buffers.read().await;
        buffers.get(stage_id).map(|b| b.len()).unwrap_or(0)
    }

    pub async fn get_stage_utilization(&self, stage_id: &str) -> f64 {
        let buffers = self.stage_buffers.read().await;
        buffers.get(stage_id).map(|b| b.utilization()).unwrap_or(0.0)
    }

    pub async fn get_global_utilization(&self) -> f64 {
        let total = *self.total_records.read().await;
        total as f64 / self.max_total_records as f64
    }

    pub async fn get_total_buffered(&self) -> usize {
        *self.total_records.read().await
    }

    pub async fn get_stages_with_data(&self) -> Vec<String> {
        let buffers = self.stage_buffers.read().await;
        buffers
            .iter()
            .filter(|(_, b)| b.len() > 0)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::with_limits(100_000, 10_000, 5_000, 0.8)
    }
}
