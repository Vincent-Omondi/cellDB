//! Data validation logic for Data Cells

use crate::schema::{SchemaDefinition, FieldType, ValidationRule};
use serde_json::Value;

pub struct Validator;

impl Validator {
    /// Validate data against schema
    pub fn validate_data(schema: &SchemaDefinition, data: &Value) -> Result<(), ValidationError> {
        // TODO: Implement comprehensive validation
        // - Type checking
        // - Required field validation
        // - Custom validation rules
        // - Constraint checking

        match data {
            Value::Object(obj) => {
                for (field_name, field_def) in &schema.fields {
                    if field_def.required && !obj.contains_key(field_name) {
                        return Err(ValidationError::MissingRequiredField(field_name.clone()));
                    }

                    if let Some(field_value) = obj.get(field_name) {
                        Self::validate_field(field_value, &field_def.field_type, &field_def.validation_rules)?;
                    }
                }
                Ok(())
            },
            _ => Err(ValidationError::InvalidDataFormat("Expected object".to_string()))
        }
    }

    /// Validate individual field
    fn validate_field(value: &Value, field_type: &FieldType, rules: &[ValidationRule]) -> Result<(), ValidationError> {
        // TODO: Implement field-level validation
        match field_type {
            FieldType::Text => {
                if !value.is_string() {
                    return Err(ValidationError::TypeMismatch("Expected string".to_string()));
                }
            },
            FieldType::Number => {
                if !value.is_number() {
                    return Err(ValidationError::TypeMismatch("Expected number".to_string()));
                }
            },
            FieldType::Boolean => {
                if !value.is_boolean() {
                    return Err(ValidationError::TypeMismatch("Expected boolean".to_string()));
                }
            },
            _ => {} // TODO: Implement other types
        }

        // Apply validation rules
        for rule in rules {
            Self::apply_validation_rule(value, rule)?;
        }

        Ok(())
    }

    /// Apply validation rule to value
    fn apply_validation_rule(value: &Value, rule: &ValidationRule) -> Result<(), ValidationError> {
        // TODO: Implement validation rules
        match rule {
            ValidationRule::MinLength(min_len) => {
                if let Value::String(s) = value {
                    if s.len() < *min_len as usize {
                        return Err(ValidationError::ValidationFailed(
                            format!("String too short, minimum length: {}", min_len)
                        ));
                    }
                }
            },
            ValidationRule::MaxLength(max_len) => {
                if let Value::String(s) = value {
                    if s.len() > *max_len as usize {
                        return Err(ValidationError::ValidationFailed(
                            format!("String too long, maximum length: {}", max_len)
                        ));
                    }
                }
            },
            _ => {} // TODO: Implement other rules
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    MissingRequiredField(String),
    TypeMismatch(String),
    ValidationFailed(String),
    InvalidDataFormat(String),
    ConstraintViolation(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingRequiredField(field) =>
                write!(f, "Missing required field: {}", field),
            ValidationError::TypeMismatch(msg) =>
                write!(f, "Type mismatch: {}", msg),
            ValidationError::ValidationFailed(msg) =>
                write!(f, "Validation failed: {}", msg),
            ValidationError::InvalidDataFormat(msg) =>
                write!(f, "Invalid data format: {}", msg),
            ValidationError::ConstraintViolation(msg) =>
                write!(f, "Constraint violation: {}", msg),
        }
    }
}