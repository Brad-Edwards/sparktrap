//! Cloud-native packet capture engine core components

pub mod buffer_manager;
pub mod capture_config;
pub mod capture_engine;
pub mod capture_error;
pub mod capture_session;
pub mod capture_statistics;
pub mod health_monitor;
pub mod interface_manager;
pub mod packet_filter;
pub mod packet_processor;
pub mod protocol_filter;
pub mod state_machine;
pub mod state_recovery;
pub mod state_sync;
pub mod state_validator;
pub mod transaction;

pub use buffer_manager::{
    Buffer, BufferManager, BufferMemory, BufferMemoryType, BufferMetadata, BufferMetrics,
    BufferState,
};
pub use capture_config::{
    CaptureConfiguration, CloudConfiguration, PerformanceConfiguration, SecurityConfiguration,
};
pub use capture_engine::CaptureEngine;
pub use capture_error::{CaptureError, CaptureErrorKind, CaptureResult};
pub use capture_session::{
    CaptureSession, SessionAction, SessionConfiguration, SessionState, SessionStats,
    SessionValidationConfig,
};
pub use capture_statistics::{
    CaptureStatistics, FlowMetrics, StateSyncMetrics, StateTransitionMetrics,
};
pub use health_monitor::{
    HealthEvent, HealthMetrics, HealthStatus, HealthThresholds, MonitoredComponent,
};
pub use interface_manager::{InterfaceManager, InterfaceState, ManagedInterface};
pub use packet_filter::{FilterRule, PacketFilter};
pub use packet_processor::PacketProcessor;
pub use protocol_filter::ProtocolFilter;
pub use state_machine::{StateMachine, StateTransition};
pub use state_recovery::{RecoveryPoint, StateRecoveryManager, StateSnapshot};
pub use state_sync::{StateChangeEvent, StateObserver, StateSync};
pub use state_validator::{StateValidator, ValidationResult, ValidationRule, ValidationSeverity};
pub use transaction::{TransactionContext, TransactionOperation, TransactionState};

// Prelude module for commonly used types
pub mod prelude {
    pub use super::CaptureConfiguration;
    pub use super::CaptureError;
    pub use super::CaptureErrorKind;
    pub use super::CaptureSession;

    // State management prelude
    pub use super::StateMachine;
    pub use super::StateRecoveryManager;
    pub use super::StateSync;
    pub use super::StateTransition;
    pub use super::StateValidator;
    pub use super::ValidationSeverity;

    // Packet processing
    pub use super::PacketFilter;
    pub use super::PacketProcessor;
    pub use super::ProtocolFilter;
}

// Optional feature-gated components
#[cfg(feature = "advanced_state_management")]
pub mod advanced_state {
    //! Advanced state management features

    pub use super::state_machine::StateMachineBuilder;
    pub use super::state_recovery::StateRecoveryConfig;
    pub use super::state_sync::{ConsistencyChecker, SyncStrategy};
    pub use super::state_validator::{CustomValidator, InvariantChecker, ValidationRuleBuilder};
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Build timestamp
pub fn build_timestamp() -> String {
    std::env::var("BUILD_TIMESTAMP").unwrap_or_else(|_| "unknown".to_string())
}
/// Git commit hash
pub fn commit_hash() -> String {
    std::env::var("GIT_COMMIT_HASH").unwrap_or_else(|_| "unknown".to_string())
}

/// Feature flags for state management
#[cfg(feature = "state_management")]
pub mod state_features {
    pub use super::state_machine::*;
    pub use super::state_recovery::*;
    pub use super::state_sync::*;
    pub use super::state_validator::*;
}
