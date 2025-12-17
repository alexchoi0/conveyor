use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTimestamp {
    pub seconds: i64,
    pub nanos: i32,
}

impl From<prost_types::Timestamp> for SerializableTimestamp {
    fn from(ts: prost_types::Timestamp) -> Self {
        Self {
            seconds: ts.seconds,
            nanos: ts.nanos,
        }
    }
}

impl From<SerializableTimestamp> for prost_types::Timestamp {
    fn from(ts: SerializableTimestamp) -> Self {
        Self {
            seconds: ts.seconds,
            nanos: ts.nanos,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouterCommand {
    Noop,

    RegisterService {
        service_id: String,
        service_name: String,
        service_type: String,
        endpoint: String,
        labels: HashMap<String, String>,
        group_id: Option<String>,
    },

    DeregisterService {
        service_id: String,
    },

    RenewLease {
        service_id: String,
    },

    UpdateServiceHealth {
        service_id: String,
        health: String,
    },

    CreatePipeline {
        pipeline_id: String,
        name: String,
        config: Vec<u8>,
    },

    UpdatePipeline {
        pipeline_id: String,
        config: Vec<u8>,
    },

    DeletePipeline {
        pipeline_id: String,
    },

    EnablePipeline {
        pipeline_id: String,
    },

    DisablePipeline {
        pipeline_id: String,
    },

    CommitSourceOffset {
        source_id: String,
        partition: u32,
        offset: u64,
    },

    AdvanceWatermark {
        source_id: String,
        partition: u32,
        position: u64,
        event_time: Option<SerializableTimestamp>,
    },

    SaveServiceCheckpoint {
        service_id: String,
        checkpoint_id: String,
        data: Vec<u8>,
        source_offsets: HashMap<String, u64>,
    },

    JoinGroup {
        service_id: String,
        group_id: String,
        stage_id: String,
    },

    LeaveGroup {
        service_id: String,
        group_id: String,
    },

    AssignPartitions {
        group_id: String,
        assignments: HashMap<String, Vec<u32>>,
        generation: u64,
    },

    CommitGroupOffset {
        group_id: String,
        source_id: String,
        partition: u32,
        offset: u64,
    },
}
