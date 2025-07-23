//! State management for Cell Manager canister using stable memory

use candid::Principal;
use ic_stable_structures::{StableBTreeMap, DefaultMemoryImpl, RestrictedMemory, memory_manager::{MemoryManager, MemoryId}};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::types::*;

type Memory = RestrictedMemory<DefaultMemoryImpl>;
type CellStorage = StableBTreeMap<Principal, CellInfo, Memory>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CELLS: RefCell<CellStorage> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

pub struct State;

impl State {
    /// Initialize the state
    pub fn init() {
        // State initialization is handled by thread_local initialization
    }

    /// Pre-upgrade hook
    pub fn pre_upgrade() {
        // Stable structures automatically handle persistence
    }

    /// Post-upgrade hook
    pub fn post_upgrade() {
        // Stable structures automatically handle restoration
    }

    /// Register a new cell
    pub fn register_cell(cell_id: Principal, cell_info: CellInfo) {
        CELLS.with(|cells| {
            cells.borrow_mut().insert(cell_id, cell_info);
        });
    }

    /// Get cell information
    pub fn get_cell(cell_id: &Principal) -> Option<CellInfo> {
        CELLS.with(|cells| {
            cells.borrow().get(cell_id)
        })
    }

    /// List all cells
    pub fn list_all_cells() -> Vec<(Principal, CellInfo)> {
        CELLS.with(|cells| {
            cells.borrow().iter().collect()
        })
    }
}