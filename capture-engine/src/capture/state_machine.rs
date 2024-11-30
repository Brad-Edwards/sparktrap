#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]
// capture-engine/src/capture/capture_error.rs
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicU64;
use std::time::SystemTime;

use crate::capture::capture_error::CaptureError;
use crate::capture::capture_error::CaptureErrorKind;

/// Represents a generic state transition event
#[derive(Debug, Clone)]
pub struct StateTransition<S> {
    from: S,
    to: S,
    timestamp: SystemTime,
    reason: Option<String>,
}

/// Core state machine implementation
#[derive(Debug)]
pub struct StateMachine<S>
where
    S: Clone + Eq + std::hash::Hash,
{
    current_state: S,
    allowed_transitions: HashMap<S, Vec<S>>,
    transition_history: VecDeque<StateTransition<S>>,
    max_history: usize,
}

impl<S> StateMachine<S>
where
    S: Clone + Eq + std::hash::Hash,
{
    /// Creates a new state machine with initial state
    pub fn new(initial_state: S, max_history: usize) -> Self {
        unimplemented!()
    }

    /// Adds allowed transition between states
    pub fn add_transition(&mut self, from: S, to: S) {
        unimplemented!()
    }

    /// Attempts to transition to new state
    pub fn transition_to(
        &mut self,
        new_state: S,
        reason: Option<String>,
    ) -> Result<(), CaptureError> {
        unimplemented!()
    }

    /// Gets current state
    pub fn current_state(&self) -> &S {
        unimplemented!()
    }

    /// Gets transition history
    pub fn history(&self) -> &VecDeque<StateTransition<S>> {
        unimplemented!()
    }

    /// Checks if a transition is valid
    pub fn can_transition_to(&self, state: &S) -> bool {
        unimplemented!()
    }
}

/// Metrics for state machine monitoring
pub struct StateMetrics {
    transitions_count: AtomicU64,
    failed_transitions: AtomicU64,
    average_transition_time: AtomicU64,
}

/// Builder pattern for state machine configuration
#[derive(Default)]
pub struct StateMachineBuilder<S>
where
    S: Clone + Eq + std::hash::Hash,
{
    initial_state: Option<S>,
    max_history: usize,
    transitions: Vec<(S, S)>,
}

impl<S> StateMachineBuilder<S>
where
    S: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn initial_state(mut self, state: S) -> Self {
        unimplemented!()
    }

    pub fn max_history(mut self, size: usize) -> Self {
        unimplemented!()
    }

    pub fn add_transition(mut self, from: S, to: S) -> Self {
        unimplemented!()
    }

    pub fn build(self) -> Result<StateMachine<S>, CaptureError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::sync::Arc;
    use std::thread;
    use std::time::SystemTime;

    // Helper enum for testing
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    enum TestState {
        Initial,
        Processing,
        Complete,
        Error,
        Pending,
        Canceled,
        Reviewing,
        Approved,
        Rejected,
    }

    // Test fixture setup
    fn setup() -> StateMachine<TestState> {
        let mut sm = StateMachine::new(TestState::Initial, 5);
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);
        sm.add_transition(TestState::Processing, TestState::Error);
        sm
    }

    #[test]
    fn test_new_state_machine_initialization() {
        let sm = StateMachine::new(TestState::Initial, 5);
        assert_eq!(*sm.current_state(), TestState::Initial);
        assert_eq!(sm.history().len(), 0);
    }

    #[test]
    fn test_add_valid_transition() {
        let mut sm = setup();
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
        let mut sm = StateMachine::new(TestState::Initial, 2); // Small history size
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);

        sm.transition_to(TestState::Processing, Some("First".to_string()))
            .unwrap();
        sm.transition_to(TestState::Complete, Some("Second".to_string()))
            .unwrap();
        sm.transition_to(TestState::Processing, Some("Third".to_string()))
            .unwrap();

        assert_eq!(sm.history().len(), 2); // Should only keep last 2 transitions
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
        let sm = StateMachine::new(TestState::Initial, 5);
        assert!(!sm.can_transition_to(&TestState::Processing));
    }

    #[test]
    fn test_transition_timestamp() {
        let mut sm = setup();
        let before = SystemTime::now();
        sm.transition_to(TestState::Processing, None).unwrap();
        let after = SystemTime::now();

        let transition = sm.history().front().unwrap();
        assert!(transition.timestamp >= before && transition.timestamp <= after);
    }

    #[test]
    fn test_concurrent_transitions() {
        let sm = Arc::new(parking_lot::Mutex::new(StateMachine::new(
            TestState::Initial,
            100,
        )));

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
        let mut sm = StateMachine::new(TestState::Initial, 1000);

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
        let mut sm = StateMachine::new(TestState::Initial, 100);

        // Create a complex state graph
        sm.add_transition(TestState::Initial, TestState::Pending);
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Pending, TestState::Processing);
        sm.add_transition(TestState::Pending, TestState::Canceled);
        sm.add_transition(TestState::Processing, TestState::Reviewing);
        sm.add_transition(TestState::Processing, TestState::Error);
        sm.add_transition(TestState::Reviewing, TestState::Approved);
        sm.add_transition(TestState::Reviewing, TestState::Rejected);
        sm.add_transition(TestState::Approved, TestState::Complete);
        sm.add_transition(TestState::Rejected, TestState::Processing);
        sm.add_transition(TestState::Error, TestState::Initial);

        // Test different paths through the graph
        let paths = vec![
            vec![
                TestState::Pending,
                TestState::Processing,
                TestState::Reviewing,
                TestState::Approved,
                TestState::Complete,
            ],
            vec![TestState::Processing, TestState::Error, TestState::Initial],
            vec![TestState::Pending, TestState::Canceled],
            vec![
                TestState::Processing,
                TestState::Reviewing,
                TestState::Rejected,
                TestState::Processing,
            ],
        ];

        for path in paths {
            let mut sm = StateMachine::new(TestState::Initial, 100);
            for state in path {
                let result = sm.transition_to(state.clone(), None);
                assert!(
                    result.is_ok(),
                    "Failed to transition to {:?} from {:?}",
                    state,
                    sm.current_state()
                );
            }
        }
    }

    #[test]
    fn test_transition_after_history_clear() {
        let mut sm = StateMachine::new(TestState::Initial, 5);
        sm.add_transition(TestState::Initial, TestState::Processing);
        sm.add_transition(TestState::Processing, TestState::Complete);

        // Fill history
        for _ in 0..5 {
            sm.transition_to(TestState::Processing, None).unwrap();
            sm.add_transition(TestState::Processing, TestState::Initial);
            sm.transition_to(TestState::Initial, None).unwrap();
        }

        assert_eq!(sm.history().len(), 5);

        // Perform another transition
        sm.transition_to(
            TestState::Processing,
            Some("After history full".to_string()),
        )
        .unwrap();

        // Verify oldest entry was removed
        assert_eq!(sm.history().len(), 5);
        assert_eq!(sm.history().back().unwrap().reason, None);
        assert_eq!(
            sm.history().front().unwrap().reason,
            Some("After history full".to_string())
        );
    }

    #[test]
    fn test_stress_with_mixed_operations() {
        let mut sm = StateMachine::new(TestState::Initial, 100);

        // Add all possible transitions
        for from in [
            TestState::Initial,
            TestState::Processing,
            TestState::Reviewing,
        ] {
            for to in [
                TestState::Processing,
                TestState::Reviewing,
                TestState::Complete,
            ] {
                sm.add_transition(from.clone(), to.clone());
            }
        }

        // Mix of valid and invalid transitions
        let operations = vec![
            (TestState::Processing, true), // valid
            (TestState::Complete, false),  // invalid from Initial
            (TestState::Reviewing, true),  // valid
            (TestState::Initial, false),   // invalid backward transition
            (TestState::Error, false),     // undefined transition
            (TestState::Processing, true), // valid
            (TestState::Complete, true),   // valid
        ];

        for (state, should_succeed) in operations {
            let result = sm.transition_to(state.clone(), None);
            assert_eq!(
                result.is_ok(),
                should_succeed,
                "Transition to {:?} should_succeed: {}",
                state,
                should_succeed
            );
        }
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
        let result = StateMachineBuilder::<TestState>::new()
            .max_history(5)
            .build();

        assert!(result.is_err());
        match result {
            Err(err) if matches!(err.kind(), CaptureErrorKind::Configuration(_)) => {
                assert!(err.to_string().contains("initial state"));
            }
            _ => panic!("Expected Configuration error"),
        }
    }

    #[test]
    fn test_builder_zero_history_size() {
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(0)
            .build();

        assert!(result.is_err());
        match result {
            Err(err) if matches!(err.kind(), CaptureErrorKind::Configuration(_)) => {
                assert!(err.to_string().contains("history size"));
            }
            _ => panic!("Expected Configuration error"),
        }
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
        let result = StateMachineBuilder::new()
            .initial_state(TestState::Initial)
            .max_history(usize::MAX)
            .build();

        assert!(result.is_err());
        match result {
            Err(err) if matches!(err.kind(), CaptureErrorKind::Configuration(_)) => {
                assert!(err.to_string().contains("history size"));
            }
            _ => panic!("Expected Configuration error"),
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
}
