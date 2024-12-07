// state/traits.rs
/// `StateManager` oversees system state transitions and persists state across restarts if needed.
use async_trait::async_trait;
use std::collections::HashMap;

use crate::traits::{Error, EventHandler, HealthCheck, HealthStatus, Lifecycle, PressureLevel};

/// Events related to state changes.
#[derive(Debug)]
pub enum StateEvent {
    StateChange(State),
    ComponentStateChange(ComponentStateChange),
    PressureStateChange(PressureState),
}

/// Trait for managing the overall state of the system.
#[async_trait]
pub trait StateManager: Lifecycle + EventHandler<StateEvent> + HealthCheck + Send + Sync {
    /// Retrieves the current system state.
    fn system_state(&self) -> SystemState;

    /// Persists the current state.
    async fn persist_state(&self) -> Result<(), Error>;

    /// Requests a state transition.
    async fn request_state_transition(&mut self, transition: StateTransition) -> Result<(), Error>;

    /// Handles pressure changes in the system.
    async fn handle_pressure_change(&mut self, pressure_state: PressureState) -> Result<(), Error>;
}

/// Represents the overall system state.
#[derive(Debug, Clone)]
pub struct SystemState {
    pub capture_state: CaptureState,
    pub component_states: HashMap<String, ComponentState>,
    pub pressure_state: PressureState,
}

/// States of the capture process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureState {
    Initializing,
    Ready,
    Capturing,
    Paused,
    ShuttingDown,
    Error(String),
}

/// State of individual components.
#[derive(Debug, Clone)]
pub struct ComponentState {
    pub name: String,
    pub status: ComponentStatus,
    pub health: HealthStatus,
    pub last_updated: u64,
}

/// Status of a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentStatus {
    Starting,
    Running,
    Degraded,
    Failed,
    Stopped,
}

/// Represents a change in a component's state.
#[derive(Debug, Clone)]
pub struct ComponentStateChange {
    pub component_name: String,
    pub new_state: ComponentState,
}

/// Represents the pressure state of the system.
#[derive(Debug, Clone)]
pub struct PressureState {
    pub memory: PressureLevel,
    pub cpu: PressureLevel,
    pub network: PressureLevel,
    pub storage: PressureLevel,
}

/// Represents a request to transition the system state.
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from_state: CaptureState,
    pub to_state: CaptureState,
    pub reason: String,
}

#[derive(Debug)]
pub struct State;
