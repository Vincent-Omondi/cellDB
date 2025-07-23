//! # CellDB Query Aggregator
//!
//! Intelligent coordination layer for cross-cell queries with streaming interfaces,
//! cost optimization, and intelligent batching. Designed to minimize expensive
//! inter-canister calls while providing SQL-like query capabilities across
//! multiple Data Cells.

use candid::{CandidType, Principal};
use ic_cdk::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};

mod streaming;
mod coordination;
mod optimization;

use streaming::*;
use coordination::*;
use optimization::*;

/// Initialize Query Aggregator with cell registry and optimization parameters
#[init]
fn init(config: AggregatorConfig) {
    ic_cdk::println!("Initializing Query Aggregator: {}", config.name);

    // Initialize coordination state and optimization engine
    Coordination::init(&config.registered_cells);
    StreamingEngine::init(&config.streaming_config);
    QueryOptimizer::init(&config.optimization_config);
}

/// Execute streaming query across multiple Data Cells
#[update]
async fn execute_streaming_query(query_plan: QueryPlan) -> Result<StreamHandle, QueryError> {
    let caller = caller();

    ic_cdk::println!("Executing streaming query from principal: {}", caller);

    // Validate query permissions and cell access
    if !Coordination::validate_cell_access(caller, &query_plan.target_cells).await {
        return Err(QueryError::PermissionDenied("Insufficient cell access permissions".to_string()));
    }

    // Optimize query execution plan
    let optimized_plan = QueryOptimizer::optimize_plan(query_plan).await
        .map_err(|e| QueryError::OptimizationFailed(e.to_string()))?;

    // Create streaming execution context
    let stream_handle = StreamingEngine::create_stream(optimized_plan).await
        .map_err(|e| QueryError::ExecutionFailed(e.to_string()))?;

    Ok(stream_handle)
}

/// Execute batch query with intelligent coordination
#[update]
async fn execute_batch_query(query: BatchQuery) -> Result<BatchQueryResult, QueryError> {
    let caller = caller();

    ic_cdk::println!("Executing batch query across {} cells", query.target_cells.len());

    // Coordinate execution across multiple cells with optimal batching
    let coordination_result = Coordination::execute_coordinated_query(caller, query).await
        .map_err(|e| QueryError::CoordinationFailed(e.to_string()))?;

    // Apply post-processing and result aggregation
    let aggregated_result = QueryOptimizer::aggregate_results(coordination_result).await
        .map_err(|e| QueryError::AggregationFailed(e.to_string()))?;

    Ok(aggregated_result)
}

/// Get next batch of streaming results
#[update]
async fn get_stream_batch(stream_handle: StreamHandle, batch_size: u32) -> Result<StreamBatch, QueryError> {
    // Validate stream handle and fetch next batch
    StreamingEngine::get_next_batch(stream_handle, batch_size).await
        .map_err(|e| QueryError::StreamingFailed(e.to_string()))
}

/// Close streaming query and cleanup resources
#[update]
async fn close_stream(stream_handle: StreamHandle) -> Result<(), QueryError> {
    StreamingEngine::close_stream(stream_handle).await
        .map_err(|e| QueryError::StreamingFailed(e.to_string()))
}

/// Register new Data Cell for aggregation
#[update]
async fn register_cell(cell_info: CellRegistration) -> Result<(), QueryError> {
    let caller = caller();

    // Validate caller has permission to register cells
    if !Coordination::is_authorized_manager(caller).await {
        return Err(QueryError::PermissionDenied("Only authorized managers can register cells".to_string()));
    }

    Coordination::register_cell(cell_info).await
        .map_err(|e| QueryError::RegistrationFailed(e.to_string()))
}

/// Get aggregator performance metrics and health status
#[query]
fn get_aggregator_metrics() -> AggregatorMetrics {
    AggregatorMetrics {
        active_streams: StreamingEngine::get_active_stream_count(),
        registered_cells: Coordination::get_registered_cell_count(),
        query_cache_hits: QueryOptimizer::get_cache_hit_rate(),
        average_query_latency: QueryOptimizer::get_average_latency(),
        cycle_efficiency_score: QueryOptimizer::get_cycle_efficiency(),
        last_updated: api::time(),
    }
}

/// Get query execution statistics
#[query]
fn get_query_stats(time_window: u64) -> QueryStats {
    QueryOptimizer::get_execution_stats(time_window)
}

#[pre_upgrade]
fn pre_upgrade() {
    Coordination::pre_upgrade();
    StreamingEngine::pre_upgrade();
    QueryOptimizer::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    Coordination::post_upgrade();
    StreamingEngine::post_upgrade();
    QueryOptimizer::post_upgrade();
}

/// Configuration for Query Aggregator initialization
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AggregatorConfig {
    pub name: String,
    pub registered_cells: Vec<CellRegistration>,
    pub streaming_config: StreamingConfig,
    pub optimization_config: OptimizationConfig,
}

/// Cell registration information
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CellRegistration {
    pub cell_id: Principal,
    pub name: String,
    pub schema_version: u32,
    pub capabilities: Vec<CellCapability>,
    pub performance_hints: PerformanceHints,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CellCapability {
    FullTextSearch,
    GeospatialQueries,
    AdvancedIndexing,
    StreamingSupport,
    BatchOperations,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PerformanceHints {
    pub typical_response_time_ms: u32,
    pub max_concurrent_queries: u32,
    pub preferred_batch_size: u32,
    pub subnet_location: Option<String>,
}

/// Query execution plan with optimization hints
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct QueryPlan {
    pub id: String,
    pub query_type: QueryType,
    pub target_cells: Vec<Principal>,
    pub operations: Vec<QueryOperation>,
    pub coordination_strategy: CoordinationStrategy,
    pub streaming_config: Option<StreamingConfig>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum QueryType {
    SingleCell,
    CrossCell,
    Aggregation,
    Join,
    Search,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CoordinationStrategy {
    Sequential,
    Parallel,
    AdaptiveParallel,
    PipelinedStreaming,
}

/// Batch query for coordinated execution
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BatchQuery {
    pub query_sql: String,
    pub target_cells: Vec<Principal>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub options: BatchQueryOptions,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BatchQueryOptions {
    pub max_results: Option<u64>,
    pub timeout_ms: Option<u64>,
    pub consistency_level: ConsistencyLevel,
    pub result_format: ResultFormat,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ConsistencyLevel {
    Strong,
    Eventual,
    Weak,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ResultFormat {
    Json,
    Binary,
    Streaming,
}

/// Handle for managing streaming queries
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StreamHandle {
    pub id: String,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Batch of streaming results
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct StreamBatch {
    pub stream_handle: StreamHandle,
    pub batch_number: u32,
    pub records: Vec<serde_json::Value>,
    pub has_more: bool,
    pub estimated_remaining: Option<u64>,
}

/// Result of batch query execution
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BatchQueryResult {
    pub query_id: String,
    pub execution_time_ms: u64,
    pub records: Vec<serde_json::Value>,
    pub total_count: u64,
    pub cell_statistics: HashMap<Principal, CellExecutionStats>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CellExecutionStats {
    pub response_time_ms: u64,
    pub records_returned: u64,
    pub cycles_consumed: u64,
    pub cache_hit: bool,
}

/// Performance metrics for the aggregator
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AggregatorMetrics {
    pub active_streams: u32,
    pub registered_cells: u32,
    pub query_cache_hits: f64,
    pub average_query_latency: u64,
    pub cycle_efficiency_score: f64,
    pub last_updated: u64,
}

/// Query execution statistics
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct QueryStats {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub average_execution_time: u64,
    pub cache_hit_rate: f64,
    pub most_queried_cells: Vec<(Principal, u64)>,
}

/// Query aggregator errors
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum QueryError {
    PermissionDenied(String),
    OptimizationFailed(String),
    ExecutionFailed(String),
    CoordinationFailed(String),
    AggregationFailed(String),
    StreamingFailed(String),
    RegistrationFailed(String),
    InvalidQuery(String),
    CellUnavailable(Principal),
    TimeoutExceeded,
    ResourceExhausted,
}

ic_cdk::export_candid!();