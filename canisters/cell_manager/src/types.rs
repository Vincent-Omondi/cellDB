//! Type definitions for Cell Manager

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for creating a new Data Cell
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CellConfig {
    pub name: String,
    pub schema: SchemaDefinition,
    pub memory_limit: Option<u64>,
    pub cycles_limit: Option<u64>,
    pub permissions: PermissionConfig,
    pub scaling_config: Option<ScalingConfig>,
}

/// Schema definition for a Data Cell
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SchemaDefinition {
    pub version: u32,
    pub fields: HashMap<String, FieldType>,
    pub indexes: Vec<String>,
    pub constraints: Vec<SchemaConstraint>,
}

/// Field type definitions
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum FieldType {
    Text { max_length: Option<u32> },
    Number { min: Option<i64>, max: Option<i64> },
    Boolean,
    Principal,
    Timestamp,
    Blob { max_size: Option<u64> },
    Array { element_type: Box<FieldType>, max_items: Option<u32> },
    Object { fields: HashMap<String, FieldType> },
}

/// Schema constraints
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SchemaConstraint {
    Required(String),
    Unique(String),
    Index(String),
    ForeignKey { field: String, references: String },
}

/// Permission configuration
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PermissionConfig {
    pub read: Vec<AccessLevel>,
    pub write: Vec<AccessLevel>,
    pub admin: Vec<Principal>,
}

/// Access levels
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum AccessLevel {
    Public,
    Authenticated,
    Principal(Principal),
    Role(String),
}

/// Scaling configuration
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ScalingConfig {
    pub auto_scale: bool,
    pub max_cells: u32,
    pub split_threshold: f64,
    pub strategy: ScalingStrategy,
}

/// Scaling strategies
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ScalingStrategy {
    Horizontal,
    Vertical,
    Hybrid,
}

/// Information about a Data Cell
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CellInfo {
    pub id: Principal,
    pub name: String,
    pub schema: SchemaDefinition,
    pub status: CellStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub metrics: CellMetrics,
}

/// Cell status
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CellStatus {
    Creating,
    Active,
    Scaling,
    Maintenance,
    Error(String),
}

/// Cell performance metrics
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CellMetrics {
    pub memory_usage: u64,
    pub cycle_consumption: u64,
    pub operation_count: u64,
    pub last_updated: u64,
}

/// Cell Manager errors
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CellError {
    NotFound(String),
    InvalidSchema(String),
    InsufficientCycles,
    PermissionDenied,
    NotImplemented(String),
}