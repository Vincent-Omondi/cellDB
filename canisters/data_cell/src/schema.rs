//! Schema management and validation for Data Cells

use candid::{CandidType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema definition for a Data Cell
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SchemaDefinition {
    pub version: u32,
    pub name: String,
    pub fields: HashMap<String, FieldDefinition>,
    pub indexes: Vec<IndexDefinition>,
    pub constraints: Vec<ConstraintDefinition>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FieldDefinition {
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Timestamp,
    Principal,
    Blob,
    Array(Box<FieldType>),
    Object(HashMap<String, FieldDefinition>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub unique: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ConstraintDefinition {
    Unique(Vec<String>),
    ForeignKey {
        fields: Vec<String>,
        references: String,
    },
    Check(String),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ValidationRule {
    MinLength(u32),
    MaxLength(u32),
    Pattern(String),
    Range(i64, i64),
    Custom(String),
}

impl SchemaDefinition {
    /// Validate data against this schema
    pub fn validate(&self, data: &serde_json::Value) -> Result<(), String> {
        // TODO: Implement schema validation
        // - Check required fields
        // - Validate field types
        // - Apply validation rules
        Ok(())
    }

    /// Check if schema can be upgraded to new version
    pub fn can_upgrade_to(&self, new_schema: &SchemaDefinition) -> Result<(), String> {
        // TODO: Implement schema compatibility check
        // - Ensure backward compatibility
        // - Check for breaking changes
        Ok(())
    }

    /// Get field definition by name
    pub fn get_field(&self, field_name: &str) -> Option<&FieldDefinition> {
        self.fields.get(field_name)
    }

    /// List all indexed fields
    pub fn get_indexed_fields(&self) -> Vec<&str> {
        self.indexes.iter()
            .flat_map(|idx| idx.fields.iter())
            .map(|s| s.as_str())
            .collect()
    }
}