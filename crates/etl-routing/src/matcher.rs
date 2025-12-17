use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use regex::Regex;

use etl_proto::common::Record;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    RecordType(String),
    MetadataMatch { key: String, pattern: String },
    MetadataExists(String),
    MetadataEquals { key: String, value: String },
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
    Always,
    Never,
}

impl Condition {
    pub fn evaluate(&self, record: &Record) -> bool {
        match self {
            Condition::RecordType(expected) => {
                record.record_type == *expected
            }
            Condition::MetadataMatch { key, pattern } => {
                if let Some(value) = record.metadata.get(key) {
                    if let Ok(re) = Regex::new(pattern) {
                        re.is_match(value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Condition::MetadataExists(key) => {
                record.metadata.contains_key(key)
            }
            Condition::MetadataEquals { key, value } => {
                record.metadata.get(key).map(|v| v == value).unwrap_or(false)
            }
            Condition::And(conditions) => {
                conditions.iter().all(|c| c.evaluate(record))
            }
            Condition::Or(conditions) => {
                conditions.iter().any(|c| c.evaluate(record))
            }
            Condition::Not(condition) => {
                !condition.evaluate(record)
            }
            Condition::Always => true,
            Condition::Never => false,
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Always
    }
}

pub struct ConditionMatcher {
    compiled_patterns: HashMap<String, Regex>,
}

impl ConditionMatcher {
    pub fn new() -> Self {
        Self {
            compiled_patterns: HashMap::new(),
        }
    }

    pub fn precompile(&mut self, condition: &Condition) {
        match condition {
            Condition::MetadataMatch { pattern, .. } => {
                if !self.compiled_patterns.contains_key(pattern) {
                    if let Ok(re) = Regex::new(pattern) {
                        self.compiled_patterns.insert(pattern.clone(), re);
                    }
                }
            }
            Condition::And(conditions) | Condition::Or(conditions) => {
                for c in conditions {
                    self.precompile(c);
                }
            }
            Condition::Not(condition) => {
                self.precompile(condition);
            }
            _ => {}
        }
    }

    pub fn evaluate_with_cache(&self, condition: &Condition, record: &Record) -> bool {
        match condition {
            Condition::MetadataMatch { key, pattern } => {
                if let Some(value) = record.metadata.get(key) {
                    if let Some(re) = self.compiled_patterns.get(pattern) {
                        re.is_match(value)
                    } else if let Ok(re) = Regex::new(pattern) {
                        re.is_match(value)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Condition::And(conditions) => {
                conditions.iter().all(|c| self.evaluate_with_cache(c, record))
            }
            Condition::Or(conditions) => {
                conditions.iter().any(|c| self.evaluate_with_cache(c, record))
            }
            Condition::Not(inner) => {
                !self.evaluate_with_cache(inner, record)
            }
            _ => condition.evaluate(record),
        }
    }
}

impl Default for ConditionMatcher {
    fn default() -> Self {
        Self::new()
    }
}
