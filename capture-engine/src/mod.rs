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
