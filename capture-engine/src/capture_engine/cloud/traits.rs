// cloud/traits.rs
use crate::traits::{Error, EventHandler, HealthCheck, Lifecycle, ValidationResult};
/// The `CloudManager` handles cloud-specific integration.
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// Events specific to cloud management.
#[derive(Debug)]
pub enum CloudEvent {
    Lifecycle(CloudLifecycleEvent),
    Network(NetworkEvent),
    Resource(ResourceEvent),
}

/// Cloud lifecycle events.
#[derive(Debug)]
pub enum CloudLifecycleEvent {
    InstanceStart(InstanceMetadata),
    InstanceStop,
    InstanceTerminate,
    InstancePreempt(Duration),
}

/// Network-related events.
#[derive(Debug)]
pub enum NetworkEvent {
    MirrorSessionUpdate(MirrorSessionConfig),
    InterfaceChange(NetworkInterfaceUpdate),
    BandwidthChange(BandwidthLimit),
}

/// Resource-related events.
#[derive(Debug)]
pub enum ResourceEvent {
    MemoryLimit(u64),
    CpuLimit(u32),
    StorageLimit(u64),
}

/// Trait for cloud integration.
#[async_trait]
pub trait CloudManager: Lifecycle + EventHandler<CloudEvent> + HealthCheck + Send + Sync {
    /// Retrieves instance metadata.
    fn instance_metadata(&self) -> Result<InstanceMetadata, Error>;

    /// Configures a mirror session.
    async fn configure_mirror_session(&mut self, config: MirrorSessionConfig) -> Result<(), Error>;

    /// Validates a mirror session configuration.
    fn validate_mirror_session(&self, config: &MirrorSessionConfig) -> ValidationResult;

    /// Retrieves resource limits of the instance.
    fn resource_limits(&self) -> Result<InstanceLimits, Error>;

    /// Retrieves network capabilities.
    fn network_capabilities(&self) -> Result<NetworkCapabilities, Error>;
}

/// Metadata about the instance.
#[derive(Debug, Clone)]
pub struct InstanceMetadata {
    pub instance_id: String,
    pub instance_type: String,
    pub availability_zone: String,
    pub tags: HashMap<String, String>,
}

/// Configuration for a mirror session.
#[derive(Debug, Clone)]
pub struct MirrorSessionConfig {
    pub session_id: String,
    pub source: String,
    pub target: String,
    pub filter_rules: Vec<FilterRule>,
}

/// Update to a network interface.
#[derive(Debug, Clone)]
pub struct NetworkInterfaceUpdate {
    pub interface_id: String,
    pub status: String,
}

/// Bandwidth limit for the instance or interface.
#[derive(Debug, Clone)]
pub struct BandwidthLimit {
    pub limit_mbps: u32,
}

/// Limits on instance resources.
#[derive(Debug, Clone)]
pub struct InstanceLimits {
    pub max_memory_mb: u64,
    pub max_storage_gb: u64,
    pub max_cpu_cores: u32,
}

/// Capabilities of the network.
#[derive(Debug, Clone)]
pub struct NetworkCapabilities {
    pub max_bandwidth_mbps: u32,
    pub supported_protocols: Vec<String>,
}

/// Filter rule for mirror sessions.
#[derive(Debug, Clone)]
pub struct FilterRule {
    pub protocol: String,
    pub source_ip: Option<IpAddr>,
    pub dest_ip: Option<IpAddr>,
    pub source_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub action: FilterAction,
}

/// Actions for filter rules.
#[derive(Debug, Clone)]
pub enum FilterAction {
    Accept,
    Drop,
    Mirror,
}
