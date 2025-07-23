//! Stable memory storage implementation for Data Cells

use ic_stable_structures::{
    StableBTreeMap, StableVec, DefaultMemoryImpl, RestrictedMemory,
    memory_manager::{MemoryManager, MemoryId}
};
use std::cell::RefCell;
use crate::schema::SchemaDefinition;

type Memory = RestrictedMemory<DefaultMemoryImpl>;
type RecordStorage = StableBTreeMap<String, Vec<u8>, Memory>;
type IndexStorage = StableBTreeMap<String, Vec<String>, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static RECORDS: RefCell<RecordStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static INDEXES: RefCell<IndexStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );
}

pub struct Storage;

impl Storage {
    /// Initialize storage with schema
    pub fn init(schema: &SchemaDefinition) {
        ic_cdk::println!("Initializing storage for schema: {}", schema.name);
        // TODO: Initialize indexes based on schema
    }

    /// Store a record
    pub fn store_record(record_id: String, data: Vec<u8>) -> Result<(), String> {
        RECORDS.with(|records| {
            records.borrow_mut().insert(record_id, data);
            Ok(())
        })
    }

    /// Retrieve a record
    pub fn get_record(record_id: &str) -> Option<Vec<u8>> {
        RECORDS.with(|records| {
            records.borrow().get(record_id)
        })
    }

    /// Delete a record
    pub fn delete_record(record_id: &str) -> Option<Vec<u8>> {
        RECORDS.with(|records| {
            records.borrow_mut().remove(record_id)
        })
    }

    /// Update index for a field
    pub fn update_index(field_name: String, field_value: String, record_id: String) {
        let index_key = format!("{}:{}", field_name, field_value);

        INDEXES.with(|indexes| {
            let mut indexes_ref = indexes.borrow_mut();
            let mut record_ids = indexes_ref.get(&index_key).unwrap_or_default();

            if !record_ids.contains(&record_id) {
                record_ids.push(record_id);
                indexes_ref.insert(index_key, record_ids);
            }
        });
    }

    /// Query records by index
    pub fn query_by_index(field_name: &str, field_value: &str) -> Vec<String> {
        let index_key = format!("{}:{}", field_name, field_value);

        INDEXES.with(|indexes| {
            indexes.borrow().get(&index_key).unwrap_or_default()
        })
    }

    /// Get storage statistics
    pub fn get_stats() -> StorageStats {
        let record_count = RECORDS.with(|records| records.borrow().len());
        let index_count = INDEXES.with(|indexes| indexes.borrow().len());

        StorageStats {
            record_count,
            index_count,
            memory_usage: 0, // TODO: Calculate actual memory usage
        }
    }

    pub fn pre_upgrade() {
        // Stable structures handle persistence automatically
    }

    pub fn post_upgrade() {
        // Stable structures handle restoration automatically
    }
}

pub struct StorageStats {
    pub record_count: u64,
    pub index_count: u64,
    pub memory_usage: u64,
}