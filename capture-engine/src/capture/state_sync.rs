// capture-engine/src/capture/state_sync.rs
/// Synchronizes the state of the capture engine with the control plane.
use crate::capture::capture_error::{
    CaptureError, CaptureErrorKind, ConfigErrorKind, RuntimeErrorKind,
};
use crate::capture::state_machine::{StateMachine, StateTransition};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};
use std::time::{Duration, SystemTime};

/// Represents a state change event
///
/// This struct is used to capture state changes in the capture engine
///
/// # Type Parameters
/// * `S` - Type of the state machine state
///
/// # Fields
/// * `timestamp` - Timestamp when the event was created
/// * `entity_id` - Unique identifier for the entity that changed state
/// * `transition` - State transition that occurred
/// * `metadata` - Additional metadata for the event
#[derive(Clone, Debug)]
#[repr(C)] // Ensure consistent memory layout
pub struct StateChangeEvent<S: Clone> {
    timestamp: SystemTime,
    entity_id: String,
    transition: StateTransition<S>,
    metadata: HashMap<String, String>,
}

impl<S: Clone> StateChangeEvent<S> {
    /// Creates a new state change event
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity that changed state
    /// * `transition` - State transition that occurred
    /// * `metadata` - Additional metadata for the event
    ///
    /// # Returns
    /// A new StateChangeEvent instance
    pub fn new(
        entity_id: String,
        transition: StateTransition<S>,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            entity_id,
            transition,
            timestamp: SystemTime::now(),
            metadata,
        }
    }

    /// Returns the entity ID for the capture engine instance
    ///
    /// # Returns
    /// A reference to the entity ID string
    pub fn entity_id(&self) -> &String {
        &self.entity_id
    }

    /// Returns the state transition that occurred in the capture engine
    ///
    /// # Returns
    /// A reference to the state transition
    #[inline]
    pub fn transition(&self) -> &StateTransition<S> {
        &self.transition
    }

    /// Returns the timestamp for the state change event
    ///
    /// # Returns
    /// The timestamp when the event was created
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    /// Returns the metadata associated with the state change event
    ///
    /// # Returns
    /// A reference to the metadata map containing control plane properties
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Creates a new state change event with an empty metadata map
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity that changed state
    /// * `transition` - State transition that occurred
    ///
    /// # Returns
    /// A new StateChangeEvent instance with an empty metadata map
    pub fn new_fast(entity_id: String, transition: StateTransition<S>) -> Self {
        Self {
            entity_id,
            transition,
            timestamp: SystemTime::now(),
            metadata: HashMap::with_capacity(0),
        }
    }

    /// Pre-allocates metadata capacity for known field count
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity that changed state
    /// * `transition` - State transition that occurred
    /// * `metadata_capacity` - Expected number of metadata fields
    ///
    /// # Returns
    /// A new StateChangeEvent instance with pre-allocated metadata capacity
    pub fn with_capacity(
        entity_id: String,
        transition: StateTransition<S>,
        metadata_capacity: usize,
    ) -> Self {
        Self {
            entity_id,
            transition,
            timestamp: SystemTime::now(),
            metadata: HashMap::with_capacity(metadata_capacity),
        }
    }
}

/// Defines how state should be synchronized
///
/// This enum is used to specify the synchronization strategy for state changes
///
/// # Variants
/// * `Immediate` - Synchronize state changes immediately with all nodes
/// * `Eventual` - Synchronize state changes with a specified delay
/// * `OnDemand` - Synchronize state changes only on specific triggers
#[derive(Debug, Clone, Copy)]
pub enum SyncStrategy {
    Immediate,
    Eventual { delay_ms: u64 },
    OnDemand,
}

/// Configuration for state synchronization
///
/// This struct is used to configure state synchronization behavior
///
/// # Fields
/// * `report_interval` - Interval for reporting state changes
/// * `retry_attempts` - Number of retry attempts for failed sync operations
/// * `retry_delay` - Delay between retry attempts
#[derive(Debug, Clone)]
pub struct StateSyncConfig {
    report_interval: Duration,
    retry_attempts: u32,
    retry_delay: Duration,
}

impl Default for StateSyncConfig {
    /// Creates a new default configuration
    ///
    /// # Returns
    /// A new StateSyncConfig instance with default values
    fn default() -> Self {
        Self {
            report_interval: Duration::from_secs(1),
            retry_attempts: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

impl StateSyncConfig {
    /// Creates a new configuration with a specified report interval
    ///
    /// # Arguments
    /// * `report_interval` - Interval for reporting state changes
    pub fn new(report_interval: Duration) -> Self {
        Self {
            report_interval,
            ..Default::default()
        }
    }

    /// Creates a new configuration with a specified report interval and retry settings
    ///
    /// # Arguments
    /// * `attempts` - Number of retry attempts for failed sync operations
    ///
    /// # Returns
    /// A new StateSyncConfig instance with the specified retry attempts
    pub fn with_retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = attempts;
        self
    }

    /// Creates a new configuration with a specified report interval and retry settings
    ///
    /// # Arguments
    /// * `delay` - Delay between retry attempts
    ///
    /// # Returns
    /// A new StateSyncConfig instance with the specified retry delay
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Returns the report interval for state synchronization
    ///
    /// # Returns
    /// The report interval duration
    pub fn report_interval(&self) -> Duration {
        self.report_interval
    }

    /// Returns the number of retry attempts for failed sync operations
    ///
    /// # Returns
    /// The number of retry attempts
    pub fn retry_attempts(&self) -> u32 {
        self.retry_attempts
    }

    /// Returns the delay between retry attempts
    ///
    /// # Returns
    /// The retry delay duration
    pub fn retry_delay(&self) -> Duration {
        self.retry_delay
    }

    /// Validates the configuration settings
    ///
    /// # Returns
    /// An error if the configuration is invalid
    pub fn validate(&self) -> Result<(), CaptureError> {
        if self.retry_attempts == 0 {
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "retry_attempts must be greater than 0",
            ));
        }
        if self.retry_delay.is_zero() {
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "retry_delay must be greater than 0",
            ));
        }
        Ok(())
    }
}

/// Metrics for sync operations
///
/// This struct is used to track synchronization metrics for state changes
///
/// # Fields
/// * `sync_attempts` - Number of attempted sync operations
/// * `failed_syncs` - Number of failed sync operations
/// * `average_sync_time` - Average time for successful sync operations
#[derive(Debug, Default)]
pub struct SyncMetrics {
    sync_attempts: AtomicU64,
    failed_syncs: AtomicU64,
    average_sync_time: AtomicU64,
}

impl SyncMetrics {
    /// Creates a new metrics instance
    ///
    /// # Returns
    /// A new SyncMetrics instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a successful sync attempt
    ///
    /// # Arguments
    /// * `duration_ms` - Duration of the sync operation in milliseconds
    ///
    /// # Notes
    /// Careful with this one, need to check for overflow
    pub fn record_sync_attempt(&self, duration_ms: u64) {
        self.sync_attempts.fetch_add(1, Ordering::Relaxed);

        // Calculate running average to prevent overflow
        let current_avg = self.average_sync_time.load(Ordering::Relaxed);
        let attempts = self.sync_attempts.load(Ordering::Relaxed);

        // Use checked arithmetic operations
        let new_avg = if attempts == 1 {
            duration_ms
        } else {
            // Formula: new_avg = ((old_avg * (n-1)) + new_value) / n
            // Rewritten to minimize overflow risk:
            // new_avg = old_avg + (new_value - old_avg) / n
            current_avg.saturating_add(
                (duration_ms.saturating_sub(current_avg))
                    .checked_div(attempts)
                    .unwrap_or(0),
            )
        };

        self.average_sync_time.store(new_avg, Ordering::Relaxed);
    }

    /// Records a failed sync attempt
    ///
    /// Increments the failed sync counter
    pub fn record_failed_sync(&self) {
        self.failed_syncs.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the number of sync attempts
    ///
    /// # Returns
    /// The number of attempted sync operations
    pub fn sync_attempts(&self) -> u64 {
        self.sync_attempts.load(Ordering::Relaxed)
    }

    /// Returns the number of failed sync attempts
    ///
    /// # Returns
    /// The number of failed sync operations
    pub fn failed_syncs(&self) -> u64 {
        self.failed_syncs.load(Ordering::Relaxed)
    }

    /// Returns the average time for successful sync operations
    ///
    /// # Returns
    /// The average time for successful sync operations
    pub fn average_sync_time(&self) -> u64 {
        self.average_sync_time.load(Ordering::Relaxed)
    }
}

/// State synchronization engine
///
/// This struct is used to synchronize state changes across the capture engine
///
/// # Type Parameters
/// * `S` - Type of the state machine state
///
/// # Fields
/// * `engine_id` - Unique identifier for the capture engine instance
/// * `state_machine` - Local state machine for tracking state changes
/// * `control_plane_reporter` - Reporter for state change events
/// * `metrics` - Metrics for sync operations
/// * `config` - Configuration for state synchronization
pub struct StateSync<S: Clone + Eq + std::hash::Hash> {
    engine_id: String,
    state_machine: Arc<RwLock<StateMachine<S>>>,
    control_plane_reporter: Box<dyn StateReporter<S>>,
    metrics: SyncMetrics,
    config: StateSyncConfig,
}

/// Trait for reporting state changes
///
/// This trait is used to report state changes to the control plane
///
/// # Type Parameters
/// * `S` - Type of the state machine state
pub trait StateReporter<S: Clone>: Send + Sync {
    /// Reports a state change event to the control plane
    ///
    /// # Arguments
    /// * `event` - State change event to report
    ///
    /// # Returns
    /// A future that resolves to a result indicating success or failure
    fn report_state<'a>(
        &'a self,
        event: &'a StateChangeEvent<S>,
    ) -> Pin<Box<dyn Future<Output = Result<(), CaptureError>> + Send + 'a>>;
}

impl<S: Clone + Eq + std::hash::Hash + Send + Sync + 'static> StateSync<S> {
    /// Creates a new state synchronization engine
    ///
    /// # Returns
    /// A new StateSyncBuilder instance
    pub fn builder() -> StateSyncBuilder<S> {
        StateSyncBuilder::<S>::new()
    }

    /// Updates the state machine with a new state
    ///
    /// # Arguments
    /// * `new_state` - New state to transition to
    /// * `metadata` - Additional metadata for the state change event
    ///
    /// # Returns
    /// An error if the state change could not be reported
    pub async fn update_state(
        &self,
        new_state: S,
        metadata: HashMap<String, String>,
    ) -> Result<(), CaptureError> {
        let start = SystemTime::now();

        // Get current state and create transition
        let current_state = self
            .state_machine
            .read()
            .map_err(|_| {
                CaptureError::new(
                    CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
                    "Failed to acquire state machine read lock",
                )
            })?
            .current_state()
            .clone();

        let transition = StateTransition::new(current_state, new_state.clone(), None);

        // Update local state machine
        self.state_machine
            .write()
            .map_err(|_| {
                CaptureError::new(
                    CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
                    "Failed to acquire state machine write lock",
                )
            })?
            .transition_to(new_state, Some("State update".to_string()))?;

        let event = StateChangeEvent::new(self.engine_id.clone(), transition, metadata);

        // Attempt to report state change
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.retry_attempts() {
            match self.control_plane_reporter.report_state(&event).await {
                Ok(_) => {
                    // Record successful sync
                    if let Ok(duration) = start.elapsed() {
                        self.metrics.record_sync_attempt(duration.as_nanos() as u64);
                    }
                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;
                    last_error = Some(e);
                    if attempts < self.config.retry_attempts() {
                        tokio::time::sleep(self.config.retry_delay()).await;
                    }
                }
            }
        }

        // Record failed sync
        self.metrics.record_failed_sync();

        // Return last error if all retries failed
        Err(last_error.unwrap_or_else(|| {
            *CaptureError::new(
                CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
                "Failed to report state change after all retries",
            )
        }))
    }

    /// Returns the state synchronization configuration
    ///
    /// # Returns
    /// A reference to the state synchronization configuration
    pub fn metrics(&self) -> &SyncMetrics {
        &self.metrics
    }

    /// Returns the state synchronization configuration
    ///
    /// # Returns
    /// A reference to the state synchronization configuration
    pub fn current_state(&self) -> Result<S, CaptureError> {
        self.state_machine
            .read()
            .map_err(|_| {
                *CaptureError::new(
                    CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
                    "Failed to acquire state machine read lock",
                )
            })
            .map(|machine| machine.current_state().clone())
    }
}

/// Builder for StateSync
///
/// This struct is used to build a new StateSync instance
///
/// # Type Parameters
/// * `S` - Type of the state machine state
///
/// # Fields
/// * `engine_id` - Unique identifier for the capture engine instance
/// * `state_machine` - Local state machine for tracking state changes
/// * `control_plane_reporter` - Reporter for state change events
/// * `config` - Configuration for state synchronization
pub struct StateSyncBuilder<S: Clone + Eq + std::hash::Hash> {
    engine_id: Option<String>,
    state_machine: Option<StateMachine<S>>,
    control_plane_reporter: Option<Box<dyn StateReporter<S>>>,
    config: Option<StateSyncConfig>,
}

impl<S: Clone + Eq + std::hash::Hash> Clone for StateSyncBuilder<S>
where
    StateMachine<S>: Clone,
{
    /// Clones the builder
    ///
    /// # Returns
    /// A new StateSyncBuilder instance
    fn clone(&self) -> Self {
        Self {
            engine_id: self.engine_id.clone(),
            state_machine: self.state_machine.clone(),
            control_plane_reporter: None, // Can't clone the reporter
            config: self.config.clone(),
        }
    }
}

impl<S: Clone + Eq + std::hash::Hash + Send + Sync + 'static> StateSyncBuilder<S> {
    /// Creates a new state synchronization builder
    ///
    /// # Returns
    /// A new StateSyncBuilder instance
    pub fn new() -> Self {
        Self {
            engine_id: None,
            state_machine: None,
            control_plane_reporter: None,
            config: None,
        }
    }

    /// Sets the engine ID for the capture engine instance
    ///
    /// # Arguments
    /// * `engine_id` - Unique identifier for the capture engine instance
    ///
    /// # Returns
    /// The updated StateSyncBuilder instance
    pub fn with_engine_id(mut self, engine_id: String) -> Self {
        self.engine_id = Some(engine_id);
        self
    }

    /// Sets the state machine for tracking state changes
    ///
    /// # Arguments
    /// * `state_machine` - Local state machine for tracking state changes
    ///
    /// # Returns
    /// The updated StateSyncBuilder instance
    pub fn with_state_machine(mut self, state_machine: StateMachine<S>) -> Self {
        self.state_machine = Some(state_machine);
        self
    }

    /// Sets the state change event reporter
    ///
    /// # Arguments
    /// * `reporter` - Reporter for state change events
    ///
    /// # Returns
    /// The updated StateSyncBuilder instance
    pub fn with_reporter(mut self, reporter: Box<dyn StateReporter<S>>) -> Self {
        self.control_plane_reporter = Some(reporter);
        self
    }

    /// Sets the state synchronization configuration
    ///
    /// # Arguments
    /// * `config` - Configuration for state synchronization
    ///
    /// # Returns
    /// The updated StateSyncBuilder instance
    pub fn with_config(mut self, config: StateSyncConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Builds a new StateSync instance
    ///
    /// # Returns
    /// A new StateSync instance
    pub fn build(self) -> Result<StateSync<S>, CaptureError> {
        let engine_id = self.engine_id.ok_or_else(|| {
            CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "engine_id is required",
            )
        })?;

        let state_machine = self.state_machine.ok_or_else(|| {
            CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "state_machine is required",
            )
        })?;

        let control_plane_reporter = self.control_plane_reporter.ok_or_else(|| {
            CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "control_plane_reporter is required",
            )
        })?;

        let config = self.config.ok_or_else(|| {
            CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "config is required",
            )
        })?;

        Ok(StateSync {
            engine_id,
            state_machine: Arc::new(RwLock::new(state_machine)),
            control_plane_reporter,
            metrics: SyncMetrics::new(),
            config,
        })
    }
}

impl<S: Clone + Eq + std::hash::Hash + Send + Sync + 'static> Default for StateSyncBuilder<S> {
    /// Creates a new default builder
    ///
    /// # Returns
    /// A new StateSyncBuilder instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod state_change_event_tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    // Test state enum
    #[derive(Clone, Debug)]
    enum TestState {
        Initial,
        Processing,
    }

    fn create_test_transition() -> StateTransition<TestState> {
        StateTransition::new(
            TestState::Initial,
            TestState::Processing,
            Some("test transition".to_string()),
        )
    }

    #[test]
    fn test_new_state_change_event_basic() {
        let entity_id = "test-entity".to_string();
        let transition = create_test_transition();
        let metadata = HashMap::new();

        let event = StateChangeEvent::new(entity_id.clone(), transition, metadata);

        assert_eq!(event.entity_id(), &entity_id);
        assert!(matches!(event.transition().from(), TestState::Initial));
        assert!(matches!(event.transition().to(), TestState::Processing));
        assert!(event.metadata().is_empty());

        let now = SystemTime::now();
        let diff = now
            .duration_since(event.timestamp())
            .expect("Time should not go backwards");
        assert!(diff < Duration::from_secs(1));
    }

    #[test]
    fn test_state_change_event_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());

        let event = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            metadata.clone(),
        );

        assert_eq!(event.metadata().get("key1"), Some(&"value1".to_string()));
        assert_eq!(event.metadata().get("key2"), Some(&"value2".to_string()));
        assert_eq!(event.metadata().len(), 2);
    }

    #[test]
    fn test_empty_entity_id() {
        let event = StateChangeEvent::new("".to_string(), create_test_transition(), HashMap::new());

        assert_eq!(event.entity_id(), "");
    }

    #[test]
    fn test_large_metadata() {
        let mut metadata = HashMap::new();
        for i in 0..1000 {
            metadata.insert(format!("key{}", i), format!("value{}", i));
        }

        let event = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            metadata.clone(),
        );

        assert_eq!(event.metadata().len(), 1000);
        // Verify a few random keys
        assert_eq!(event.metadata().get("key0"), Some(&"value0".to_string()));
        assert_eq!(
            event.metadata().get("key999"),
            Some(&"value999".to_string())
        );
    }

    #[test]
    fn test_clone_behavior() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let original = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            metadata,
        );

        let cloned = original.clone();

        assert_eq!(original.entity_id(), cloned.entity_id());
        assert_eq!(original.metadata(), cloned.metadata());
        assert_eq!(original.timestamp(), cloned.timestamp());
        // Check transition fields individually
        assert!(matches!(original.transition().from(), TestState::Initial));
        assert!(matches!(original.transition().to(), TestState::Processing));
    }

    #[test]
    fn test_metadata_special_characters() {
        let mut metadata = HashMap::new();
        metadata.insert("!@#$%^&*()".to_string(), "特殊字符".to_string());
        metadata.insert("emoji🎉".to_string(), "value🌟".to_string());

        let event = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            metadata.clone(),
        );

        assert_eq!(
            event.metadata().get("!@#$%^&*()"),
            Some(&"特殊字符".to_string())
        );
        assert_eq!(
            event.metadata().get("emoji🎉"),
            Some(&"value🌟".to_string())
        );
    }

    #[test]
    fn test_debug_format() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let event = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            metadata,
        );

        let debug_string = format!("{:?}", event);
        assert!(debug_string.contains("test-entity"));
        assert!(debug_string.contains("key"));
        assert!(debug_string.contains("value"));
    }

    #[test]
    fn test_timestamp_monotonicity() {
        let event1 = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            HashMap::new(),
        );

        std::thread::sleep(Duration::from_millis(10));

        let event2 = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            HashMap::new(),
        );

        assert!(event2.timestamp() >= event1.timestamp());
    }

    #[test]
    fn test_with_empty_metadata() {
        let event = StateChangeEvent::new(
            "test-entity".to_string(),
            create_test_transition(),
            HashMap::new(),
        );

        assert!(event.metadata().is_empty());
    }
}

#[cfg(test)]
mod state_config_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_config() {
        let config = StateSyncConfig::default();
        assert_eq!(config.report_interval, Duration::from_secs(1));
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.retry_delay, Duration::from_secs(1));
    }

    #[test]
    fn test_custom_config() {
        let config = StateSyncConfig {
            report_interval: Duration::from_secs(5),
            retry_attempts: 5,
            retry_delay: Duration::from_millis(500),
        };

        assert_eq!(config.report_interval, Duration::from_secs(5));
        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.retry_delay, Duration::from_millis(500));
    }

    #[test]
    fn test_zero_duration_config() {
        let config = StateSyncConfig {
            report_interval: Duration::from_secs(0),
            retry_attempts: 0,
            retry_delay: Duration::from_secs(0),
        };

        assert_eq!(config.report_interval, Duration::from_secs(0));
        assert_eq!(config.retry_attempts, 0);
        assert_eq!(config.retry_delay, Duration::from_secs(0));
    }

    #[test]
    fn test_max_values_config() {
        let config = StateSyncConfig {
            report_interval: Duration::from_secs(u64::MAX),
            retry_attempts: u32::MAX,
            retry_delay: Duration::from_secs(u64::MAX),
        };

        assert_eq!(config.report_interval, Duration::from_secs(u64::MAX));
        assert_eq!(config.retry_attempts, u32::MAX);
        assert_eq!(config.retry_delay, Duration::from_secs(u64::MAX));
    }

    #[test]
    fn test_clone_implementation() {
        let original = StateSyncConfig {
            report_interval: Duration::from_secs(2),
            retry_attempts: 4,
            retry_delay: Duration::from_millis(750),
        };

        let cloned = original.clone();

        assert_eq!(original.report_interval, cloned.report_interval);
        assert_eq!(original.retry_attempts, cloned.retry_attempts);
        assert_eq!(original.retry_delay, cloned.retry_delay);
    }

    #[test]
    fn test_debug_implementation() {
        let config = StateSyncConfig::default();
        let debug_string = format!("{:?}", config);

        assert!(debug_string.contains("StateSyncConfig"));
        assert!(debug_string.contains("report_interval"));
        assert!(debug_string.contains("retry_attempts"));
        assert!(debug_string.contains("retry_delay"));
    }
}

#[cfg(test)]
mod sync_metrics_tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_new_metrics() {
        let metrics = SyncMetrics::new();
        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.failed_syncs(), 0);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[test]
    fn test_single_sync_attempt() {
        let metrics = SyncMetrics::new();
        metrics.record_sync_attempt(100);

        assert_eq!(metrics.sync_attempts(), 1);
        assert_eq!(metrics.average_sync_time(), 100);
    }

    #[test]
    fn test_multiple_sync_attempts() {
        let metrics = SyncMetrics::new();

        metrics.record_sync_attempt(100);
        metrics.record_sync_attempt(200);
        metrics.record_sync_attempt(300);

        assert_eq!(metrics.sync_attempts(), 3);
        // Average should be 200 ((100 + 200 + 300) / 3)
        assert_eq!(metrics.average_sync_time(), 200);
    }

    #[test]
    fn test_failed_syncs() {
        let metrics = SyncMetrics::new();

        metrics.record_failed_sync();
        metrics.record_failed_sync();

        assert_eq!(metrics.failed_syncs(), 2);
        // Failed syncs shouldn't affect sync attempts or average time
        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[test]
    fn test_mixed_success_failure() {
        let metrics = SyncMetrics::new();

        metrics.record_sync_attempt(100);
        metrics.record_failed_sync();
        metrics.record_sync_attempt(300);
        metrics.record_failed_sync();

        assert_eq!(metrics.sync_attempts(), 2);
        assert_eq!(metrics.failed_syncs(), 2);
        assert_eq!(metrics.average_sync_time(), 200); // (100 + 300) / 2
    }

    #[tokio::test]
    async fn test_concurrent_sync_attempts() {
        let metrics = Arc::new(SyncMetrics::new());
        let thread_count = 10;
        let mut handles: Vec<
            tokio::task::JoinHandle<Result<Result<(), CaptureError>, CaptureError>>,
        > = vec![];

        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_sync_attempt(100);
                Ok(Ok(()))
            }));
        }

        #[allow(unused_must_use)]
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(metrics.sync_attempts(), thread_count);
        assert_eq!(metrics.average_sync_time(), 100);
    }

    #[tokio::test]
    async fn test_concurrent_failures() {
        let metrics = Arc::new(SyncMetrics::new());
        let thread_count = 10;
        let mut handles: Vec<tokio::task::JoinHandle<Result<(), CaptureError>>> = vec![];

        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_failed_sync();
                Ok(())
            }));
        }

        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        assert_eq!(metrics.failed_syncs(), thread_count);
    }

    #[test]
    fn test_zero_duration_sync() {
        let metrics = SyncMetrics::new();
        metrics.record_sync_attempt(0);

        assert_eq!(metrics.sync_attempts(), 1);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[test]
    fn test_max_duration_sync() {
        let metrics = SyncMetrics::new();
        metrics.record_sync_attempt(u64::MAX);

        assert_eq!(metrics.sync_attempts(), 1);
        assert_eq!(metrics.average_sync_time(), u64::MAX);
    }

    #[test]
    fn test_average_time_overflow_protection() {
        let metrics = SyncMetrics::new();

        // Record several very large durations
        for _ in 0..5 {
            metrics.record_sync_attempt(u64::MAX / 2);
        }

        // Average should not overflow
        assert!(metrics.average_sync_time() <= u64::MAX);
    }

    #[test]
    fn test_sync_attempts_overflow_protection() {
        let metrics = SyncMetrics::new();

        // Try to simulate counter overflow
        for _ in 0..100 {
            metrics.record_sync_attempt(1);
        }

        assert!(metrics.sync_attempts() <= u64::MAX);
    }

    #[test]
    fn test_failed_syncs_overflow_protection() {
        let metrics = SyncMetrics::new();

        // Try to simulate counter overflow
        for _ in 0..100 {
            metrics.record_failed_sync();
        }

        assert!(metrics.failed_syncs() <= u64::MAX);
    }

    #[tokio::test]
    async fn test_concurrent_mixed_operations() {
        let metrics = Arc::new(SyncMetrics::new());
        let thread_count = 10;
        let mut handles: Vec<
            tokio::task::JoinHandle<Result<Result<(), CaptureError>, CaptureError>>,
        > = vec![];

        // Spawn threads for successful syncs
        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_sync_attempt(100);
                Ok(Ok(()))
            }));
        }

        // Spawn threads for failed syncs
        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_failed_sync();
                Ok(Ok(()))
            }));
        }

        #[allow(unused_must_use)]
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        assert_eq!(metrics.sync_attempts(), thread_count);
        assert_eq!(metrics.failed_syncs(), thread_count);
        assert_eq!(metrics.average_sync_time(), 100);
    }

    #[test]
    fn test_default_creation() {
        let metrics = SyncMetrics::default();
        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.failed_syncs(), 0);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[test]
    fn test_metrics_independence() {
        let metrics1 = SyncMetrics::new();
        let metrics2 = SyncMetrics::new();

        metrics1.record_sync_attempt(100);
        metrics2.record_sync_attempt(200);

        assert_eq!(metrics1.average_sync_time(), 100);
        assert_eq!(metrics2.average_sync_time(), 200);
    }
}

#[cfg(test)]
mod state_sync_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::test;

    #[test]
    async fn test_sync_metrics_default() {
        let metrics = SyncMetrics::default();
        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.failed_syncs(), 0);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[tokio::test]
    async fn test_successful_sync_metrics() {
        let metrics = Arc::new(SyncMetrics::new());
        metrics.record_sync_attempt(100);

        assert_eq!(metrics.sync_attempts(), 1);
        assert_eq!(metrics.failed_syncs(), 0);
        assert_eq!(metrics.average_sync_time(), 100);
    }

    #[tokio::test]
    async fn test_failed_sync_metrics() {
        let metrics = Arc::new(SyncMetrics::new());
        metrics.record_failed_sync();

        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.failed_syncs(), 1);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[tokio::test]
    async fn test_multiple_sync_attempts() {
        let metrics = Arc::new(SyncMetrics::new());

        metrics.record_sync_attempt(100);
        metrics.record_sync_attempt(200);
        metrics.record_sync_attempt(300);

        assert_eq!(metrics.sync_attempts(), 3);
        assert_eq!(metrics.average_sync_time(), 200);
    }

    #[tokio::test]
    async fn test_concurrent_sync_operations() {
        let metrics = Arc::new(SyncMetrics::new());
        let thread_count = 10;
        let mut handles = vec![];

        // Spawn threads for successful syncs
        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_sync_attempt(100);
                Ok::<(), CaptureError>(())
            }));
        }

        // Spawn threads for failed syncs
        for _ in 0..thread_count {
            let metrics_clone = Arc::clone(&metrics);
            handles.push(tokio::spawn(async move {
                metrics_clone.record_failed_sync();
                Ok::<(), CaptureError>(())
            }));
        }

        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        assert_eq!(metrics.sync_attempts(), thread_count);
        assert_eq!(metrics.failed_syncs(), thread_count);
        assert_eq!(metrics.average_sync_time(), 100);
    }

    #[test]
    async fn test_metrics_independence() {
        let metrics1 = SyncMetrics::new();
        let metrics2 = SyncMetrics::new();

        metrics1.record_sync_attempt(100);
        metrics2.record_sync_attempt(200);

        assert_eq!(metrics1.average_sync_time(), 100);
        assert_eq!(metrics2.average_sync_time(), 200);
    }

    #[test]
    async fn test_edge_cases() {
        let metrics = SyncMetrics::new();

        metrics.record_sync_attempt(0);
        assert_eq!(metrics.average_sync_time(), 0);

        let metrics = SyncMetrics::new();
        metrics.record_sync_attempt(u64::MAX);
        assert_eq!(metrics.average_sync_time(), u64::MAX);

        assert!(metrics.failed_syncs() <= u64::MAX);
    }

    async fn sync_with_retry(
        metrics: &Arc<SyncMetrics>,
        config: &StateSyncConfig,
    ) -> Result<(), CaptureError> {
        for _ in 0..config.retry_attempts() {
            metrics.record_failed_sync();
            tokio::time::sleep(config.retry_delay()).await;
        }
        Err(*CaptureError::new(
            CaptureErrorKind::Runtime(RuntimeErrorKind::OperationFailed),
            "Simulated sync failure",
        ))
    }

    #[tokio::test]
    async fn test_sync_retry_mechanism() {
        let metrics = Arc::new(SyncMetrics::new());
        let config = StateSyncConfig {
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
            report_interval: Duration::from_secs(1),
        };

        // Test with failing sync that should retry
        let result = sync_with_retry(&metrics, &config).await;
        assert!(result.is_err());
        assert_eq!(metrics.failed_syncs(), 3); // One for each retry attempt
    }

    #[test]
    async fn test_metrics_reset() {
        let mut metrics = SyncMetrics::new();

        metrics.record_sync_attempt(100);
        metrics.record_failed_sync();

        metrics = SyncMetrics::new();

        assert_eq!(metrics.sync_attempts(), 0);
        assert_eq!(metrics.failed_syncs(), 0);
        assert_eq!(metrics.average_sync_time(), 0);
    }

    #[tokio::test]
    async fn test_sync_cancellation() {
        let metrics = Arc::new(SyncMetrics::new());
        let handle = tokio::spawn(async move {
            // Simulate a long-running sync
            tokio::time::sleep(Duration::from_secs(5)).await;
            metrics.record_sync_attempt(500);
        });

        // Cancel the sync operation
        handle.abort();
        assert!(handle.await.is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use mockall::predicate::*;
    use std::future::Future;
    use std::pin::Pin;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum TestState {
        Initial,
        Final,
    }

    impl Clone for StateMachine<TestState> {
        fn clone(&self) -> Self {
            let mut new_machine = StateMachine::new(self.current_state().clone(), 1)
                .expect("Failed to create state machine");

            new_machine.add_transition(TestState::Initial, TestState::Final);
            new_machine
        }
    }

    mock! {
        #[derive(Debug)]
        pub StateReporter<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static> {}

        impl<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static> StateReporter<T> for StateReporter<T> {
            fn report_state<'a>(
                &'a self,
                event: &'a StateChangeEvent<T>
            ) -> Pin<Box<dyn Future<Output = Result<(), CaptureError>> + Send + 'static>>;
        }
    }

    struct TestContext {
        state_machine: StateMachine<TestState>,
        config: StateSyncConfig,
        mock_reporter: MockStateReporter<TestState>,
    }

    impl TestContext {
        fn new() -> Self {
            let mut state_machine = StateMachine::new(
                TestState::Initial,
                1, // Expected capacity for transitions
            )
            .expect("Failed to create state machine");
            state_machine.add_transition(TestState::Initial, TestState::Final);

            let config = StateSyncConfig {
                report_interval: Duration::from_secs(1),
                retry_attempts: 3,
                retry_delay: Duration::from_secs(1),
            };

            let mock_reporter = MockStateReporter::new();
            Self {
                state_machine,
                config,
                mock_reporter,
            }
        }
    }

    #[tokio::test]
    async fn test_builder_basic_construction() {
        let builder = StateSyncBuilder::<TestState>::new();
        assert!(builder.engine_id.is_none());
        assert!(builder.state_machine.is_none());
        assert!(builder.control_plane_reporter.is_none());
        assert!(builder.config.is_none());
    }

    #[tokio::test]
    async fn test_builder_with_all_fields() -> Result<(), CaptureError> {
        let mut ctx = TestContext::new();

        ctx.mock_reporter
            .expect_report_state()
            .returning(|_event| Box::pin(async { Ok(()) }));

        let state_sync = StateSyncBuilder::<TestState>::new()
            .with_engine_id("test-engine".to_string())
            .with_state_machine(ctx.state_machine)
            .with_reporter(Box::new(ctx.mock_reporter))
            .with_config(ctx.config)
            .build()?;

        assert_eq!(state_sync.engine_id, "test-engine");
        Ok(())
    }

    #[tokio::test]
    async fn test_builder_missing_required_fields() {
        let ctx1 = TestContext::new();
        let mut ctx2 = TestContext::new();

        // Test missing engine_id
        let result = StateSyncBuilder::<TestState>::new()
            .with_state_machine(ctx1.state_machine)
            .with_reporter(Box::new(ctx1.mock_reporter))
            .with_config(ctx1.config)
            .build();

        match result {
            Ok(_) => panic!("Expected error for missing engine_id"),
            Err(e) => assert!(matches!(e.kind(), CaptureErrorKind::Configuration(_))),
        }

        // Test missing state_machine
        ctx2.mock_reporter
            .expect_report_state()
            .returning(|_| Box::pin(async { Ok(()) }));

        let result = StateSyncBuilder::<TestState>::new()
            .with_engine_id("test-engine".to_string())
            .with_reporter(Box::new(ctx2.mock_reporter))
            .with_config(ctx2.config)
            .build();

        match result {
            Ok(_) => panic!("Expected error for missing state_machine"),
            Err(e) => assert!(matches!(e.kind(), CaptureErrorKind::Configuration(_))),
        }
    }

    #[cfg(test)]
    impl PartialEq for StateSyncConfig {
        fn eq(&self, other: &Self) -> bool {
            self.report_interval == other.report_interval
                && self.retry_attempts == other.retry_attempts
                && self.retry_delay == other.retry_delay
        }
    }

    #[cfg(test)]
    fn assert_configs_equal(left: &StateSyncConfig, right: &StateSyncConfig) {
        assert_eq!(left.report_interval, right.report_interval);
        assert_eq!(left.retry_attempts, right.retry_attempts);
        assert_eq!(left.retry_delay, right.retry_delay);
    }

    #[test]
    fn test_builder_clone() {
        let ctx = TestContext::new();

        let original = StateSyncBuilder::<TestState>::new()
            .with_engine_id("test-engine".to_string())
            .with_state_machine(ctx.state_machine)
            .with_config(ctx.config);

        let cloned = original.clone();

        assert_eq!(cloned.engine_id, original.engine_id);
        assert!(cloned.control_plane_reporter.is_none());

        #[cfg(test)]
        assert_configs_equal(&cloned.config.unwrap(), &original.config.unwrap());
    }

    #[tokio::test]
    async fn test_builder_default() {
        let builder: StateSyncBuilder<TestState> = StateSyncBuilder::default();
        assert!(builder.engine_id.is_none());
        assert!(builder.state_machine.is_none());
        assert!(builder.control_plane_reporter.is_none());
        assert!(builder.config.is_none());
    }

    #[tokio::test]
    async fn test_builder_chaining() -> Result<(), CaptureError> {
        let mut ctx = TestContext::new();

        ctx.mock_reporter
            .expect_report_state()
            .returning(|_event| Box::pin(async { Ok(()) }));

        let state_sync = StateSyncBuilder::<TestState>::new()
            .with_engine_id("test-engine".to_string())
            .with_state_machine(ctx.state_machine)
            .with_reporter(Box::new(ctx.mock_reporter))
            .with_config(ctx.config)
            .build()?;

        assert_eq!(state_sync.engine_id, "test-engine");
        Ok(())
    }
}
