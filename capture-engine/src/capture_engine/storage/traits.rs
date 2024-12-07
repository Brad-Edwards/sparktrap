// storage/traits.rs
/// `StorageManager` abstracts reading, writing, and deleting data in storage.
/// The abstraction allows implementing compliance features like encryption, auditing, and WORM storage.
use async_trait::async_trait;
use bytes::Bytes;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::traits::{Error, EventHandler, Lifecycle, PressureAware, PressureStatus};

/// Events specific to storage management.
#[derive(Debug)]
pub enum StorageEvent {
    SpaceThreshold(SpaceThresholdEvent),
    WriteFailure(WriteFailureInfo),
    DeviceError(DeviceError),
    PerformanceDegraded(PerformanceInfo),
}

/// Trait for managing local storage.
#[async_trait]
pub trait StorageManager:
    Lifecycle + EventHandler<StorageEvent> + PressureAware + Send + Sync
{
    /// Writes data to storage.
    async fn write_data(&mut self, data: StorageData) -> Result<StorageId, Error>;

    /// Reads data from storage.
    async fn read_data(&mut self, id: &StorageId) -> Result<StorageData, Error>;

    /// Deletes data from storage.
    async fn delete_data(&mut self, id: &StorageId) -> Result<(), Error>;

    /// Retrieves storage pressure status.
    fn storage_pressure_status(&self) -> PressureStatus;

    /// Retrieves storage space statistics.
    fn space_stats(&self) -> SpaceStats;

    /// Flushes storage buffers.
    async fn flush(&mut self) -> Result<(), Error>;
}

/// Represents data to be stored.
#[derive(Debug, Clone)]
pub struct StorageData {
    pub data: Bytes,
    pub metadata: StorageMetadata,
}

/// Metadata associated with storage data.
#[derive(Debug, Clone)]
pub struct StorageMetadata {
    pub timestamp: u64,
    pub tags: HashMap<String, String>,
}

/// Identifier for stored data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageId(String);

/// Event when space thresholds are crossed.
#[derive(Debug)]
pub struct SpaceThresholdEvent {
    pub threshold_type: SpaceThresholdType,
    pub utilization: f32,
}

/// Types of space thresholds.
#[derive(Debug, Clone)]
pub enum SpaceThresholdType {
    Warning,
    Critical,
}

/// Information about a write failure.
#[derive(Debug)]
pub struct WriteFailureInfo {
    pub error: String,
    pub storage_id: Option<StorageId>,
}

/// Error occurring in storage devices.
#[derive(Debug)]
pub struct DeviceError {
    pub device_path: PathBuf,
    pub error: String,
}

/// Information about degraded performance.
#[derive(Debug)]
pub struct PerformanceInfo {
    pub metric: String,
    pub value: f32,
    pub threshold: f32,
}

/// Statistics about storage space.
#[derive(Debug)]
pub struct SpaceStats {
    pub total_space: u64,
    pub used_space: u64,
    pub available_space: u64,
    pub utilization_percent: f32,
}
