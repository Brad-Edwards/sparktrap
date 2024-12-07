// lib.rs
//! Capture Engine - High performance packet capture for cloud environments
//!
//! This crate provides the core packet capture engine functionality that runs on each capture node.
//! It focuses on efficient packet capture, local resource management, and communication with the
//! control plane.

pub mod capture_engine;
pub mod traits;

// Version and build information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn build_timestamp() -> String {
    std::env::var("BUILD_TIMESTAMP").unwrap_or_else(|_| "unknown".to_string())
}
pub fn git_hash() -> String {
    std::env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string())
}
