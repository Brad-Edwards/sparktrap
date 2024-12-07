// output/traits.rs
/// `OutputManager` sends processed packet data to destinations (e.g., S3 buckets, Kafka topics, local files).
use async_trait::async_trait;
use bytes::Bytes;
use std::collections::HashMap;

use crate::traits::{
    BackpressureControl, Cleanup, Error, EventHandler, Lifecycle, PressureAware, RateLimiter,
    ResourceManager,
};

/// Events specific to output management.
#[derive(Debug)]
pub enum OutputEvent {
    DestinationStatus(DestinationStatus),
    BufferThreshold(BufferThresholdEvent),
    RotationTriggered(RotationTrigger),
    WriteError(WriteFailure),
    BackpressureEvent(BackpressureStatus),
}

/// Trait for managing data output.
#[async_trait]
pub trait OutputManager:
    Lifecycle
    + EventHandler<OutputEvent>
    + PressureAware
    + RateLimiter
    + BackpressureControl
    + ResourceManager
    + Cleanup
    + Send
    + Sync
{
    async fn send_batch(&mut self, data: &[OutputData]) -> Result<(), Error>;
    async fn add_destination(&mut self, config: OutputDestinationConfig) -> Result<(), Error>;
    async fn remove_destination(&mut self, destination_id: &str) -> Result<(), Error>;
    fn destination_status(&self, destination_id: &str) -> Option<DestinationStatus>;
    async fn flush(&mut self) -> Result<(), Error>;
}

/// Represents data to be sent to an output destination.
#[derive(Debug, Clone)]
pub struct OutputData {
    pub data: Bytes,
    pub metadata: OutputMetadata,
}

/// Metadata associated with output data.
#[derive(Debug, Clone)]
pub struct OutputMetadata {
    pub timestamp: u64,
    pub routing_info: Option<RoutingInfo>,
}

/// Information for routing output data.
#[derive(Debug, Clone)]
pub struct RoutingInfo {
    pub destination_ids: Vec<String>,
}

/// Configuration for an output destination.
#[derive(Debug, Clone)]
pub struct OutputDestinationConfig {
    pub destination_id: String,
    pub destination_type: DestinationType,
    pub settings: HashMap<String, String>,
}

/// Types of output destinations.
#[derive(Debug, Clone)]
pub enum DestinationType {
    S3,
    LocalFile,
    NetworkStream,
    Kafka,
}

/// Status of an output destination.
#[derive(Debug, Clone)]
pub struct DestinationStatus {
    pub destination_id: String,
    pub status: String,
    pub last_error: Option<String>,
}

#[derive(Debug)]
pub struct BufferThresholdEvent {
    pub threshold_reached: bool,
}

#[derive(Debug)]
pub struct RotationTrigger {
    pub reason: String,
}

#[derive(Debug)]
pub struct WriteFailure {
    pub error: String,
}

#[derive(Debug)]
pub struct BackpressureStatus {
    pub active: bool,
}
