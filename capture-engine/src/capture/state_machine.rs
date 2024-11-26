#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]
// capture-engine/src/capture/capture_error.rs
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicU64;
use std::time::SystemTime;

use crate::capture::capture_error::CaptureError;

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
