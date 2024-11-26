#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]
// capture-engine/src/capture/state_validator.rs
use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::capture::capture_error::CaptureError;

pub type ValidatorFn<S> = dyn Fn(&S, &S) -> Result<bool, CaptureError> + Send + Sync;

#[derive(Clone)]
pub struct ValidationRule<S> {
    pub name: String,
    pub description: String,
    pub severity: ValidationSeverity,
    pub validator: Arc<ValidatorFn<S>>,
    pub metadata: HashMap<String, String>,
}

pub struct ValidationRuleBuilder<S> {
    name: Option<String>,
    description: Option<String>,
    severity: Option<ValidationSeverity>,
    validator: Option<Box<ValidatorFn<S>>>,
    metadata: HashMap<String, String>,
}

/// Severity levels for validation rules
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationSeverity {
    Critical, // Must pass or state transition fails
    Warning,  // Generates warning but allows transition
    Info,     // Informational validation only
}

/// Result of a validation check
#[derive(Debug, Clone)]
pub struct ValidationResult {
    rule_name: String,
    passed: bool,
    severity: ValidationSeverity,
    message: Option<String>,
    timestamp: SystemTime,
    metadata: HashMap<String, String>,
}

/// Configuration for state validation
#[derive(Clone)]
pub struct ValidatorConfig {
    enabled: bool,
    fail_fast: bool,
    validation_timeout: Duration,
    max_retries: u32,
    retry_delay: Duration,
}

/// Core state validator implementation
pub struct StateValidator<S> {
    config: ValidatorConfig,
    rules: HashMap<String, ValidationRule<S>>,
    validation_history: Vec<ValidationResult>,
    custom_validators: Vec<Box<dyn CustomValidator<S>>>,
}

/// Trait for implementing custom validators
#[async_trait]
pub trait CustomValidator<S>: Send + Sync {
    async fn validate(&self, current: &S, proposed: &S) -> Result<ValidationResult, CaptureError>;
    fn get_name(&self) -> &str;
    fn get_severity(&self) -> ValidationSeverity;
}

/// Trait for state invariant checking
pub trait InvariantChecker<S> {
    fn check_invariants(&self, state: &S) -> Result<Vec<ValidationResult>, CaptureError>;
    fn get_invariant_rules(&self) -> Vec<ValidationRule<S>>;
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<S: Clone + Send + Sync + 'static> StateValidator<S> {
    /// Creates a new StateValidator with the given configuration
    pub fn new(config: ValidatorConfig) -> Self {
        unimplemented!()
    }

    /// Adds a new validation rule
    pub fn add_rule(&mut self, rule: ValidationRule<S>) {
        unimplemented!()
    }

    /// Adds a custom validator
    pub fn add_custom_validator(&mut self, validator: Box<dyn CustomValidator<S>>) {
        unimplemented!()
    }

    /// Validates a state transition
    pub async fn validate_transition(
        &mut self,
        current_state: &S,
        proposed_state: &S,
    ) -> Result<Vec<ValidationResult>, CaptureError> {
        unimplemented!()
    }

    /// Executes a single validation rule
    async fn execute_rule(
        &self,
        rule: &ValidationRule<S>,
        current_state: &S,
        proposed_state: &S,
    ) -> Result<ValidationResult, CaptureError> {
        unimplemented!()
    }

    /// Gets validation history
    pub fn get_validation_history(&self) -> &[ValidationResult] {
        unimplemented!()
    }

    /// Clears validation history
    pub fn clear_history(&mut self) {
        unimplemented!()
    }
}

impl<S> fmt::Debug for ValidationRule<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidationRule")
            .field("name", &self.name)
            .finish() // Skip the validator field
    }
}

impl<S> Default for ValidationRuleBuilder<S> {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<S> ValidationRuleBuilder<S> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn name(mut self, name: &str) -> Self {
        unimplemented!()
    }

    pub fn description(mut self, desc: &str) -> Self {
        unimplemented!()
    }

    pub fn severity(mut self, severity: ValidationSeverity) -> Self {
        unimplemented!()
    }

    pub fn validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&S, &S) -> Result<bool, CaptureError> + Send + Sync + 'static,
    {
        unimplemented!()
    }

    pub fn metadata(mut self, key: &str, value: &str) -> Self {
        unimplemented!()
    }

    pub fn build(self) -> Result<ValidationRule<S>, CaptureError> {
        unimplemented!()
    }
}
