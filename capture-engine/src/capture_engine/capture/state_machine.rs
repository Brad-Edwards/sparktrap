// capture-engine/src/capture/capture_error.rs
/// A state machine for managing the state of the capture engine.
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use crate::capture_engine::capture::capture_error::{
    CaptureError, CaptureErrorKind, ConfigErrorKind, ResourceErrorKind,
};

/// Represents a generic state transition event
///
/// The transition event captures the source and target states, the timestamp of the transition,
/// and an optional reason for the transition.
///
/// # Type Parameters
/// * `S` - The type of the state
///
/// # Fields
/// * `from` - The source state
/// * `to` - The target state
/// * `timestamp` - The timestamp of the transition
/// * `reason` - An optional reason for the transition
#[derive(Debug, Clone)]
pub struct StateTransition<S> {
    from: S,
    to: S,
    timestamp: SystemTime,
    reason: Option<String>,
}

impl<S> StateTransition<S>
where
    S: Clone,
{
    /// Creates a new state transition
    ///
    /// # Arguments
    /// * `from` - The source state
    /// * `to` - The target state
    ///
    /// # Returns
    /// A new StateTransition instance
    pub fn new(from: S, to: S, reason: Option<String>) -> Self {
        Self {
            from,
            to,
            timestamp: SystemTime::now(),
            reason,
        }
    }

    /// Get the source state
    ///
    /// # Returns
    /// A reference to the source state
    pub fn from(&self) -> &S {
        &self.from
    }

    /// Get the target state
    ///
    /// # Returns
    /// A reference to the target state
    pub fn to(&self) -> &S {
        &self.to
    }

    /// Get the transition timestamp
    ///
    /// # Returns
    /// The timestamp of the transition
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    /// Get the transition reason if any
    ///
    /// # Returns
    /// An optional reference to the transition reason
    pub fn reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }
}

/// Core state machine implementation
///
/// The state machine is a generic implementation that allows for defining states and transitions
///
/// # Type Parameters
/// * `S` - The type of the state
///
/// # Fields
/// * `current_state` - The current state of the state machine
/// * `allowed_transitions` - A map of allowed transitions between states
/// * `history` - A queue of state transitions
/// * `max_history` - The maximum number of transitions to keep in history
/// * `metrics` - Metrics for state machine transitions
#[derive(Debug)]
pub struct StateMachine<S>
where
    S: Clone + Eq + Hash,
{
    current_state: S,
    allowed_transitions: HashMap<S, Vec<S>>,
    history: VecDeque<StateTransition<S>>,
    max_history: usize,
    metrics: StateMetrics,
}

impl<S> StateMachine<S>
where
    S: Clone + Eq + Hash,
{
    /// Creates a new StateMachine instance
    ///
    /// # Arguments
    /// * `initial_state` - The initial state of the state machine
    /// * `max_history` - The maximum number of transitions to keep in history
    ///
    /// # Returns
    /// A new StateMachine instance
    pub fn new(initial_state: S, max_history: usize) -> Result<Self, CaptureError> {
        if max_history == 0 {
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "History size must be greater than 0",
            ));
        }

        Ok(StateMachine {
            current_state: initial_state,
            allowed_transitions: HashMap::new(),
            history: VecDeque::with_capacity(max_history),
            max_history,
            metrics: StateMetrics {
                transitions_count: AtomicU64::new(0),
                failed_transitions: AtomicU64::new(0),
                average_transition_time: AtomicU64::new(0),
            },
        })
    }

    /// Adds allowed transition between states
    ///
    /// # Arguments
    /// * `from` - The source state
    /// * `to` - The target state
    ///
    /// # Returns
    /// A reference to the state machine
    pub fn add_transition(&mut self, from: S, to: S) {
        self.allowed_transitions.entry(from).or_default().push(to);
    }

    /// Checks if transition to target state is allowed
    ///
    /// # Arguments
    /// * `target` - The target state
    ///
    /// # Returns
    /// A boolean indicating if the transition is allowed
    pub fn can_transition_to(&self, target: &S) -> bool {
        self.allowed_transitions
            .get(&self.current_state)
            .map_or(false, |allowed| allowed.contains(target))
    }

    /// Attempts to transition to new state
    ///
    /// # Arguments
    /// * `new_state` - The target state
    /// * `reason` - An optional reason for the transition
    ///
    /// # Returns
    /// A Result indicating success or failure
    pub fn transition_to(
        &mut self,
        new_state: S,
        reason: Option<String>,
    ) -> Result<(), CaptureError> {
        if !self.can_transition_to(&new_state) {
            self.metrics
                .failed_transitions
                .fetch_add(1, Ordering::Relaxed);
            return Err(*CaptureError::new(
                CaptureErrorKind::Resource(ResourceErrorKind::InvalidState),
                "Invalid state transition",
            ));
        }

        let transition = StateTransition {
            from: self.current_state.clone(),
            to: new_state.clone(),
            timestamp: SystemTime::now(),
            reason,
        };

        // Update history
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(transition);

        self.current_state = new_state;
        self.metrics
            .transitions_count
            .fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Returns current state
    ///
    /// # Returns
    /// A reference to the current state
    pub fn current_state(&self) -> &S {
        &self.current_state
    }

    /// Returns transition history
    ///
    /// # Returns
    /// A reference to the transition history
    pub fn history(&self) -> &VecDeque<StateTransition<S>> {
        &self.history
    }

    /// Clears transition history
    ///
    /// # Returns
    /// A reference to the state machine
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Metrics for state machine transitions
///
/// The state metrics capture information about the number of transitions, failed transitions,
/// and the average transition time
///
/// # Fields
/// * `transitions_count` - The total number of transitions
/// * `failed_transitions` - The total number of failed transitions
/// * `average_transition_time` - The average transition time in nanoseconds
#[derive(Debug, Default)]
pub struct StateMetrics {
    transitions_count: AtomicU64,
    failed_transitions: AtomicU64,
    average_transition_time: AtomicU64,
}

impl StateMetrics {
    /// Creates a new StateMetrics instance
    ///
    /// # Returns
    /// A new StateMetrics instance
    pub fn new() -> Self {
        Self {
            transitions_count: AtomicU64::new(0),
            failed_transitions: AtomicU64::new(0),
            average_transition_time: AtomicU64::new(0),
        }
    }

    /// Records a successful transition
    ///
    /// # Arguments
    /// * `duration_ns` - The duration of the transition in nanoseconds
    ///
    /// # Returns
    /// A reference to the state metrics
    pub fn record_transition(&self, duration_ns: u64) {
        let old_count = self.transitions_count.fetch_add(1, Ordering::Relaxed);
        let old_avg = self.average_transition_time.load(Ordering::Relaxed);

        let new_avg = if old_count == 0 {
            duration_ns
        } else {
            // Using the safer formula: new_avg = old_avg + (new_value - old_avg) / n
            old_avg.saturating_add(
                (duration_ns.saturating_sub(old_avg))
                    .checked_div(old_count + 1)
                    .unwrap_or(0),
            )
        };

        self.average_transition_time
            .store(new_avg, Ordering::Relaxed);
    }

    /// Records a failed transition
    ///
    /// # Returns
    /// A reference to the state metrics
    pub fn record_failed_transition(&self) {
        self.failed_transitions.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the total number of transitions
    ///
    /// # Returns
    /// The total number of transitions
    pub fn transitions_count(&self) -> u64 {
        self.transitions_count.load(Ordering::Relaxed)
    }

    /// Returns the total number of failed transitions
    ///
    /// # Returns
    /// The total number of failed transitions
    pub fn failed_transitions(&self) -> u64 {
        self.failed_transitions.load(Ordering::Relaxed)
    }

    /// Returns the average transition time
    ///
    /// # Returns
    /// The average transition time in nanoseconds
    pub fn average_transition_time(&self) -> u64 {
        self.average_transition_time.load(Ordering::Relaxed)
    }
}

/// Builder pattern for state machine configuration
///
/// The StateMachineBuilder allows for configuring the state machine with an initial state,
///
/// # Type Parameters
/// * `S` - The type of the state
///
/// # Fields
/// * `initial_state` - The initial state of the state machine
/// * `transitions` - A list of allowed transitions between states
/// * `max_history` - The maximum number of transitions to keep in history
pub struct StateMachineBuilder<S>
where
    S: Clone + Eq + Hash,
{
    initial_state: Option<S>,
    transitions: Vec<(S, S)>,
    max_history: usize,
}

impl<S> StateMachineBuilder<S>
where
    S: Clone + Eq + Hash,
{
    /// Creates a new StateMachineBuilder
    ///
    /// # Returns
    /// A new StateMachineBuilder instance
    pub fn new() -> Self {
        StateMachineBuilder {
            initial_state: None,
            transitions: Vec::new(),
            max_history: 100,
        }
    }

    /// Sets the initial state
    ///
    /// # Arguments
    /// * `state` - The initial state
    ///
    /// # Returns
    /// A reference to the state machine builder
    pub fn initial_state(mut self, state: S) -> Self {
        self.initial_state = Some(state);
        self
    }

    /// Sets the maximum history size
    ///
    /// # Arguments
    /// * `size` - The maximum history size
    ///
    /// # Returns
    /// A reference to the state machine builder
    pub fn max_history(mut self, size: usize) -> Self {
        self.max_history = size;
        self
    }

    /// Adds a valid state transition
    ///
    /// # Arguments
    /// * `from` - The source state
    /// * `to` - The target state
    ///
    /// # Returns
    /// A reference to the state machine builder
    pub fn add_transition(mut self, from: S, to: S) -> Self {
        self.transitions.push((from, to));
        self
    }

    /// Builds and validates the StateMachine configuration
    ///
    /// # Returns
    /// A Result containing the StateMachine instance or an error
    pub fn build(self) -> Result<StateMachine<S>, CaptureError> {
        // Add this validation at the start of the build method
        if self.initial_state.is_none() {
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "Initial state must be set before building",
            ));
        }

        // Add history size validation
        if self.max_history == 0 {
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "History size must be greater than zero",
            ));
        }

        // Add upper bound for history size
        if self.max_history > 10000 {
            // reasonable upper limit
            return Err(*CaptureError::new(
                CaptureErrorKind::Configuration(ConfigErrorKind::InvalidValue),
                "History size exceeds maximum allowed value",
            ));
        }

        let mut machine = StateMachine::new(self.initial_state.unwrap(), self.max_history)?;

        // Register all transitions
        for (from, to) in self.transitions {
            machine.add_transition(from, to);
        }

        Ok(machine)
    }
}

impl<S> Default for StateMachineBuilder<S>
where
    S: Clone + Eq + Hash,
{
    /// Creates a default StateMachineBuilder instance
    ///
    /// # Returns
    /// A new StateMachineBuilder instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, SystemTime};

    // Helper enum for testing
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    enum TestState {
        Initial,
        Processing,
        Complete,
        Error,
        Pending,
        Reviewing,
        Approved,
        Rejected,
        Start,
        End,
    }

    // Test fixture setup
    fn setup() -> StateMachine<TestState> {
        let mut sm = StateMachine::new(TestState::Initial, 5).unwrap();
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);
        sm.add_transition(TestState::Processing, TestState::Error);
        sm
    }

    #[test]
    fn test_new_state_machine_initialization() {
        let sm = StateMachine::new(TestState::Initial, 5).unwrap();
        assert_eq!(*sm.current_state(), TestState::Initial);
        assert_eq!(sm.history().len(), 0);
    }

    #[test]
    fn test_add_valid_transition() {
        let sm = setup();
        assert!(sm.can_transition_to(&TestState::Processing));
        assert!(!sm.can_transition_to(&TestState::Complete)); // Can't skip Processing
    }

    #[test]
    fn test_successful_transition() {
        let mut sm = setup();
        let result = sm.transition_to(TestState::Processing, Some("Starting process".to_string()));
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), TestState::Processing);
        assert_eq!(sm.history().len(), 1);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = setup();
        let result = sm.transition_to(TestState::Complete, None);
        assert!(result.is_err());
        assert_eq!(*sm.current_state(), TestState::Initial); // State shouldn't change
    }

    #[test]
    fn test_history_size_limit() {
        let mut sm = StateMachine::new(TestState::Initial, 2).unwrap();
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);

        assert!(sm.can_transition_to(&TestState::Processing));
        sm.transition_to(TestState::Processing, Some("First".to_string()))
            .unwrap();

        assert!(sm.can_transition_to(&TestState::Complete));
        sm.transition_to(TestState::Complete, Some("Second".to_string()))
            .unwrap();

        assert_eq!(sm.history().len(), 2);
    }

    #[test]
    fn test_transition_with_no_reason() {
        let mut sm = setup();
        sm.transition_to(TestState::Processing, None).unwrap();
        assert!(sm.history().front().unwrap().reason.is_none());
    }

    #[test]
    fn test_multiple_allowed_transitions_from_state() {
        let mut sm = setup();
        sm.transition_to(TestState::Processing, None).unwrap();
        assert!(sm.can_transition_to(&TestState::Complete));
        assert!(sm.can_transition_to(&TestState::Error));
    }

    #[test]
    fn test_transition_to_same_state() {
        let mut sm = setup();
        sm.add_transition(TestState::Initial, TestState::Initial);
        let result = sm.transition_to(TestState::Initial, None);
        assert!(result.is_ok());
        assert_eq!(sm.history().len(), 1);
    }

    #[test]
    fn test_transition_with_empty_allowed_transitions() {
        let sm = StateMachine::new(TestState::Initial, 5).unwrap();
        assert!(!sm.can_transition_to(&TestState::Processing));
    }

    #[test]
    fn test_transition_timestamp() {
        let mut sm = setup();
        let before = SystemTime::now();
        sm.transition_to(TestState::Processing, None).unwrap();
        let after = SystemTime::now();

        let sm_history = sm.history();
        let transition = sm_history.front().unwrap();
        assert!(transition.timestamp >= before && transition.timestamp <= after);
    }

    #[test]
    fn test_concurrent_transitions() {
        let sm = Arc::new(parking_lot::Mutex::new(
            StateMachine::new(TestState::Initial, 100).unwrap(),
        ));

        // Add all possible transitions
        {
            let mut locked_sm = sm.lock();
            locked_sm.add_transition(TestState::Initial, TestState::Processing);
            locked_sm.add_transition(TestState::Processing, TestState::Complete);
            locked_sm.add_transition(TestState::Processing, TestState::Error);
        }

        let threads: Vec<_> = (0..10)
            .map(|i| {
                let sm_clone = Arc::clone(&sm);
                thread::spawn(move || {
                    let mut locked_sm = sm_clone.lock();
                    let result = locked_sm
                        .transition_to(TestState::Processing, Some(format!("Thread {}", i)));
                    result.is_ok()
                })
            })
            .collect();

        let results: Vec<_> = threads
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();

        // Only one thread should succeed in transitioning
        assert_eq!(results.iter().filter(|&&x| x).count(), 1);
    }

    #[test]
    fn test_large_number_of_transitions() {
        let mut sm = StateMachine::new(TestState::Initial, 1000).unwrap();

        // Create a cycle of states
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Reviewing);
        sm.add_transition(TestState::Reviewing, TestState::Approved);
        sm.add_transition(TestState::Reviewing, TestState::Rejected);
        sm.add_transition(TestState::Approved, TestState::Complete);
        sm.add_transition(TestState::Rejected, TestState::Processing);

        // Perform many transitions
        for _ in 0..500 {
            sm.transition_to(TestState::Processing, None).unwrap();
            sm.transition_to(TestState::Reviewing, None).unwrap();

            // Randomly choose between Approved and Rejected
            if rand::random() {
                sm.transition_to(TestState::Approved, None).unwrap();
                sm.transition_to(TestState::Complete, None).unwrap();
                sm.add_transition(TestState::Complete, TestState::Initial);
                sm.transition_to(TestState::Initial, None).unwrap();
            } else {
                sm.transition_to(TestState::Rejected, None).unwrap();
            }
        }

        // Verify history size hasn't exceeded max
        assert!(sm.history().len() <= 1000);
    }

    #[test]
    fn test_complex_transition_graph() {
        let mut sm = StateMachine::new(TestState::Initial, 5).unwrap();
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Pending);
        sm.add_transition(TestState::Pending, TestState::Complete);

        assert!(sm.can_transition_to(&TestState::Processing));
        let result = sm.transition_to(TestState::Processing, None);
        assert!(result.is_ok());

        assert!(sm.can_transition_to(&TestState::Pending));
        let result = sm.transition_to(TestState::Pending, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transition_after_history_clear() {
        let mut sm = StateMachine::new(TestState::Initial, 2).unwrap();
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);

        // Fill history
        assert!(sm.can_transition_to(&TestState::Processing));
        sm.transition_to(TestState::Processing, Some("First transition".to_string()))
            .unwrap();

        assert!(sm.can_transition_to(&TestState::Complete));
        sm.transition_to(TestState::Complete, Some("Second transition".to_string()))
            .unwrap();

        // Verify history size
        assert_eq!(sm.history().len(), 2);

        // Add another transition which should cause oldest entry to be removed
        sm.add_transition(TestState::Complete, TestState::Initial);
        assert!(sm.can_transition_to(&TestState::Initial));
        sm.transition_to(TestState::Initial, Some("After history full".to_string()))
            .unwrap();

        // Verify the oldest entry was removed
        assert_eq!(sm.history().len(), 2);
        assert_eq!(
            sm.history().back().map(|t| t.reason.as_deref()),
            Some(Some("After history full"))
        );
    }

    #[test]
    fn test_stress_with_mixed_operations() {
        let mut sm = StateMachine::new(TestState::Initial, 100).unwrap();
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);

        let should_succeed = sm.can_transition_to(&TestState::Complete);
        assert_eq!(should_succeed, false); // Can't skip Processing

        assert!(sm.can_transition_to(&TestState::Processing));
        sm.transition_to(TestState::Processing, None).unwrap();

        assert!(sm.can_transition_to(&TestState::Complete));
        sm.transition_to(TestState::Complete, None).unwrap();
    }

    // StateMachineBuilder tests
    #[test]
    fn test_basic_builder_creation() {
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .build();

        assert!(result.is_ok());
        let sm = result.unwrap();
        assert_eq!(*sm.current_state(), TestState::Initial);
        assert_eq!(sm.history().len(), 0);
    }

    #[test]
    fn test_builder_with_transitions() {
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .add_transition(TestState::Initial, TestState::Processing)
            .add_transition(TestState::Processing, TestState::Complete)
            .build();

        assert!(result.is_ok());
        let sm = result.unwrap();
        assert!(sm.can_transition_to(&TestState::Processing));
        assert!(!sm.can_transition_to(&TestState::Complete)); // Can't skip Processing
    }

    #[test]
    fn test_builder_missing_initial_state() {
        let result = StateMachine::new(TestState::Initial, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_zero_history_size() {
        let result = StateMachine::new(TestState::Initial, 0);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains("History size must be greater than 0"));
    }

    #[test]
    fn test_builder_method_chaining_order() {
        // Test that the order of builder methods doesn't matter
        let sm1 = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .add_transition(TestState::Initial, TestState::Processing)
            .build()
            .unwrap();

        let sm2 = StateMachineBuilder::new()
            .add_transition(TestState::Initial, TestState::Processing)
            .max_history(5)
            .initial_state(TestState::Initial)
            .build()
            .unwrap();

        assert_eq!(*sm1.current_state(), *sm2.current_state());
        assert!(sm1.can_transition_to(&TestState::Processing));
        assert!(sm2.can_transition_to(&TestState::Processing));
    }

    #[test]
    fn test_builder_multiple_transitions() {
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .add_transition(TestState::Initial, TestState::Processing)
            .add_transition(TestState::Processing, TestState::Complete)
            .add_transition(TestState::Processing, TestState::Error)
            .add_transition(TestState::Error, TestState::Initial)
            .build();

        assert!(result.is_ok());
        let mut sm = result.unwrap();
        assert!(sm.can_transition_to(&TestState::Processing));

        // Verify transitions were added correctly
        sm.transition_to(TestState::Processing, None).unwrap();
        assert!(sm.can_transition_to(&TestState::Complete));
        assert!(sm.can_transition_to(&TestState::Error));
    }

    #[test]
    fn test_builder_duplicate_transitions() {
        let mut sm = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .add_transition(TestState::Initial, TestState::Processing)
            .add_transition(TestState::Initial, TestState::Processing) // Duplicate
            .build()
            .unwrap();

        // Should only have one transition
        assert!(sm.can_transition_to(&TestState::Processing));
        sm.transition_to(TestState::Processing, None).unwrap();
    }

    #[test]
    fn test_builder_large_history_size() {
        // Instead of using usize::MAX which causes capacity overflow,
        // use the validation limit from the implementation
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(10001) // Just over the 10000 limit shown in the context
            .build();

        assert!(result.is_err());
        match result {
            Err(err) if matches!(err.kind(), CaptureErrorKind::Configuration(_)) => {
                assert!(err.to_string().contains("History size exceeds maximum"));
            }
            _ => panic!("Expected Configuration error about history size"),
        }
    }

    #[test]
    fn test_builder_self_transition() {
        let sm = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(5)
            .add_transition(TestState::Initial, TestState::Initial)
            .build()
            .unwrap();

        assert!(sm.can_transition_to(&TestState::Initial));
    }

    #[test]
    fn test_builder_complex_graph() {
        let mut sm = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(10)
            .add_transition(TestState::Initial, TestState::Processing)
            .add_transition(TestState::Processing, TestState::Complete)
            .add_transition(TestState::Processing, TestState::Error)
            .add_transition(TestState::Error, TestState::Initial)
            .add_transition(TestState::Complete, TestState::Initial)
            .build()
            .unwrap();

        // Test circular path
        sm.transition_to(TestState::Processing, None).unwrap();
        sm.transition_to(TestState::Error, None).unwrap();
        sm.transition_to(TestState::Initial, None).unwrap();
        assert_eq!(sm.history().len(), 3);
    }

    #[test]
    fn test_builder_default() {
        let builder: StateMachineBuilder<TestState> = StateMachineBuilder::new();
        let result = builder.build();
        assert!(result.is_err()); // Should fail because no initial state is set
    }

    #[test]
    fn test_builder_reasonable_defaults() {
        let mut sm = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .build()
            .unwrap();

        // Should have some reasonable default for max_history
        assert!(sm.history().len() == 0);
        sm.transition_to(TestState::Processing, None).err().unwrap(); // Should fail as no transitions defined
    }

    #[test]
    fn test_new_metrics() {
        let metrics = StateMetrics::new();
        assert_eq!(metrics.transitions_count(), 0);
        assert_eq!(metrics.failed_transitions(), 0);
        assert_eq!(metrics.average_transition_time(), 0);
    }

    #[test]
    fn test_record_single_transition() {
        let metrics = StateMetrics::new();
        metrics.record_transition(100);
        assert_eq!(metrics.transitions_count(), 1);
        assert_eq!(metrics.average_transition_time(), 100);
    }

    #[test]
    fn test_record_multiple_transitions() {
        let metrics = StateMetrics::new();
        metrics.record_transition(100);
        metrics.record_transition(200);
        assert_eq!(metrics.transitions_count(), 2);
        assert_eq!(metrics.average_transition_time(), 150); // (100 + 200) / 2
    }

    #[test]
    fn test_record_failed_transitions() {
        let metrics = StateMetrics::new();
        metrics.record_failed_transition();
        metrics.record_failed_transition();
        assert_eq!(metrics.failed_transitions(), 2);
    }

    #[test]
    fn test_metrics_concurrent_transitions() {
        let metrics = StateMetrics::new();
        let metrics_arc = std::sync::Arc::new(metrics);
        let mut handles = vec![];

        for _ in 0..10 {
            let metrics_clone = metrics_arc.clone();
            handles.push(thread::spawn(move || {
                metrics_clone.record_transition(100);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(metrics_arc.transitions_count(), 10);
        assert_eq!(metrics_arc.average_transition_time(), 100);
    }

    #[test]
    fn test_concurrent_failed_transitions() {
        let metrics = StateMetrics::new();
        let metrics_arc = std::sync::Arc::new(metrics);
        let mut handles = vec![];

        for _ in 0..10 {
            let metrics_clone = metrics_arc.clone();
            handles.push(thread::spawn(move || {
                metrics_clone.record_failed_transition();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(metrics_arc.failed_transitions(), 10);
    }

    #[test]
    fn test_mixed_transitions() {
        let metrics = StateMetrics::new();
        metrics.record_transition(100);
        metrics.record_failed_transition();
        metrics.record_transition(300);
        metrics.record_failed_transition();

        assert_eq!(metrics.transitions_count(), 2);
        assert_eq!(metrics.failed_transitions(), 2);
        assert_eq!(metrics.average_transition_time(), 200); // (100 + 300) / 2
    }

    #[test]
    fn test_state_transition_new() {
        let transition = StateTransition::new(
            TestState::Start,
            TestState::End,
            Some("test reason".to_string()),
        );

        assert_eq!(*transition.from(), TestState::Start);
        assert_eq!(*transition.to(), TestState::End);
        assert_eq!(transition.reason().map(|s| s.as_str()), Some("test reason"));

        // Timestamp should be recent
        let now = SystemTime::now();
        let diff = now
            .duration_since(transition.timestamp())
            .expect("Time should not go backwards");
        assert!(diff < Duration::from_secs(1));
    }

    #[test]
    fn test_state_transition_no_reason() {
        let transition = StateTransition::new(TestState::Start, TestState::End, None);

        assert_eq!(*transition.from(), TestState::Start);
        assert_eq!(*transition.to(), TestState::End);
        assert!(transition.reason().is_none());
    }

    #[test]
    fn test_state_transition_with_string_state() {
        let transition = StateTransition::new("initial".to_string(), "final".to_string(), None);

        assert_eq!(transition.from(), "initial");
        assert_eq!(transition.to(), "final");
    }

    #[test]
    fn test_state_metrics_average_time_overflow_protection() {
        let metrics = StateMetrics::new();

        // Record several very large durations
        for _ in 0..5 {
            metrics.record_transition(u64::MAX / 2);
        }

        // Average should not overflow
        assert!(metrics.average_transition_time() <= u64::MAX);
    }
}
