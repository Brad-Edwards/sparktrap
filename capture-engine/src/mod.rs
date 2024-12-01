//! Capture Engine - High performance packet capture for cloud environments
//! 
//! This crate provides the core packet capture engine functionality that runs on each capture node.
//! It focuses on efficient packet capture, local resource management, and communication with the
//! control plane.

// Re-export core types that make up our public API
pub use capture::CaptureEngine;
pub use capture::CaptureConfiguration;
pub use capture::CaptureSession;

// Error handling
pub mod error;
pub use error::{Error, Result};

// Core modules
pub mod buffer;     // Buffer and memory management
pub mod capture;    // Core capture functionality
pub mod cloud;      // AWS lifecycle integration
pub mod control;    // Control plane communication
pub mod filter;     // Packet filtering
pub mod interface;  // Network interface management
pub mod output;     // Captured data output handling
pub mod protocol;   // Protocol processing
pub mod security;   // Security and credentials
pub mod state;      // State management
pub mod storage;    // Local storage management
pub mod telemetry;  // Metrics and logging

// Version and build information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
pub const GIT_HASH: &str = env!("GIT_HASH");

// Core traits that define the engine's capabilities
pub mod traits {
    pub use crate::buffer::BufferManager;
    pub use crate::capture::CaptureManager;
    pub use crate::filter::FilterManager;
    pub use crate::interface::InterfaceManager;
    pub use crate::output::OutputManager;
    pub use crate::state::StateManager;
    pub use crate::storage::StorageManager;
}

/// Engine health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Failed(String),
}

/// Initialize the capture engine with the given configuration
/// 
/// # Arguments
/// * `config` - Engine configuration
/// 
/// # Returns
/// * `Result<CaptureEngine>` - Initialized engine or error
pub fn init(config: CaptureConfiguration) -> Result<CaptureEngine> {
    unimplemented!()
}

/// Gracefully shutdown the capture engine
/// 
/// Ensures all buffers are flushed, metrics are shipped,
/// and resources are cleaned up
pub fn shutdown() -> Result<()> {
    unimplemented!()
}

/// Get current engine health status
/// 
/// Aggregates health checks across all components
pub fn health_check() -> Result<HealthStatus> {
    unimplemented!()
}

// Testing utilities
#[cfg(test)]
pub mod test {
    pub use crate::capture::test_utils::*;
    pub use crate::interface::test_utils::*;
    pub use crate::storage::test_utils::*;
}

// Internal utilities and shared code
#[doc(hidden)]
pub(crate) mod utils;

// Feature flags
#[cfg(feature = "dpdk")]
pub use interface::dpdk;

#[cfg(feature = "xdp")]
pub use interface::xdp;

#[cfg(feature = "nvme")]
pub use storage::nvme;