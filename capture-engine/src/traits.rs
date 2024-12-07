// traits.rs
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    Initialization(String),
    Runtime(String),
    Communication(String),
    IO(std::io::Error),
    Pressure(PressureErrorKind),
    ResourceExhausted(ResourceKind),
    Validation(ValidationErrorKind),
    Performance(PerformanceIssue),
    Authentication(String),
    Authorization(String),
    Security(String),
    Configuration(String),
    NotFound(String),
    Timeout(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Initialization(msg) => write!(f, "Initialization error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Communication(msg) => write!(f, "Communication error: {}", msg),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::Pressure(kind) => write!(f, "Pressure error: {:?}", kind),
            Error::ResourceExhausted(kind) => write!(f, "Resource exhausted: {:?}", kind),
            Error::Validation(kind) => write!(f, "Validation error: {:?}", kind),
            Error::Performance(issue) => write!(f, "Performance issue: {:?}", issue),
            Error::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            Error::Authorization(msg) => write!(f, "Authorization error: {}", msg),
            Error::Security(msg) => write!(f, "Security error: {}", msg),
            Error::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Timeout(msg) => write!(f, "Timeout: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            _ => None,
        }
    }
}

/// Defines the health status of a component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Kinds of pressure errors.
#[derive(Debug)]
pub enum PressureErrorKind {
    Memory,
    CPU,
    IO,
    Network,
    Storage,
    Custom(String),
}

/// Kinds of resource exhaustion.
#[derive(Debug)]
pub enum ResourceKind {
    Memory,
    CPU,
    Disk,
    Network,
    Storage,
    Custom(String),
}

/// Kinds of validation errors.
#[derive(Debug)]
pub enum ValidationErrorKind {
    InvalidConfiguration,
    MissingRequiredField,
    Conflict,
    ConstraintViolation,
    Custom(String),
}

/// Issues related to performance.
#[derive(Debug)]
pub enum PerformanceIssue {
    HighLatency,
    LowThroughput,
    ResourceStarvation,
    Overutilization,
    Custom(String),
}

/// Represents a network packet.
#[derive(Debug, Clone)]
pub struct Packet<'a> {
    pub timestamp: u64,
    pub data: &'a [u8],
    pub metadata: PacketMetadata,
    pub buffer_id: BufferId,
}

/// Metadata associated with a packet.
#[derive(Debug, Clone)]
pub struct PacketMetadata {
    pub compact_data: u128, // Bit-packed source_ip, dest_ip, ports, protocol
    pub additional_info: HashMap<String, String>,
}

/// Identifier for a buffer in zero-copy operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferId(u64);

/// Represents the pressure status of a resource.
#[derive(Debug, Clone)]
pub struct PressureStatus {
    pub level: PressureLevel,
    pub utilization: f32,
    pub available_units: usize,
}

/// Different levels of resource pressure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PressureLevel {
    Normal,
    Elevated,
    Critical,
    Overflow,
}

/// Actions that can be taken to handle pressure.
#[derive(Debug, Clone)]
pub enum PressureAction {
    Throttle,
    DropPackets,
    BackPressure,
    ScaleUp,
    EmergencyFlush,
    Custom(String),
}

/// Thresholds for pressure levels.
#[derive(Debug, Clone)]
pub struct PressureThresholds {
    pub elevated: f32,
    pub critical: f32,
    pub overflow: f32,
}

/// Result of a configuration validation.
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

/// Validation errors encountered during configuration validation.
#[derive(Debug)]
pub enum ValidationError {
    InvalidValue { field: String, reason: String },
    MissingField { field: String },
    Conflict { fields: Vec<String>, reason: String },
    ConstraintViolation { field: String, constraint: String },
}

/// Warnings encountered during configuration validation.
#[derive(Debug)]
pub enum ValidationWarning {
    DeprecatedField { field: String, alternative: String },
    PerformanceImpact { field: String, impact: String },
    SecurityRisk { field: String, description: String },
}

/// Trait for components that have a lifecycle (initialization and shutdown).
#[async_trait]
pub trait Lifecycle: Send + Sync {
    /// Initializes the component.
    async fn initialize(&mut self) -> Result<(), Error>;

    /// Shuts down the component gracefully.
    async fn shutdown(&mut self) -> Result<(), Error>;
}

/// Trait for components that can start and stop.
#[async_trait]
pub trait StartStop: Send + Sync {
    /// Starts the component's operation.
    async fn start(&mut self) -> Result<(), Error>;

    /// Stops the component's operation.
    async fn stop(&mut self) -> Result<(), Error>;
}

/// Trait for components that can pause and resume.
#[async_trait]
pub trait PauseResume: Send + Sync {
    /// Pauses the component's operation.
    async fn pause(&mut self) -> Result<(), Error>;

    /// Resumes the component's operation.
    async fn resume(&mut self) -> Result<(), Error>;
}

/// Trait for performing health checks on components.
pub trait HealthCheck: Send + Sync {
    /// Checks the health of the component.
    fn health_check(&self) -> HealthStatus;
}

/// Trait for event handling in an event-driven architecture.
#[async_trait]
pub trait EventHandler<E>: Send + Sync {
    /// Handles an incoming event.
    async fn handle_event(&mut self, event: E) -> Result<(), Error>;
}

/// Trait for processing packets.
#[async_trait]
pub trait PacketProcessor: Send + Sync {
    /// Processes a single packet.
    async fn process_packet(&mut self, packet: &mut Packet) -> Result<(), Error>;

    /// Processes a batch of packets.
    async fn process_batch(&mut self, packets: &mut [Packet]) -> Result<(), Error>;
}

/// Trait for components aware of and reacting to resource pressure.
#[async_trait]
pub trait PressureAware: Send + Sync {
    /// Retrieves the current pressure status.
    fn pressure_status(&self) -> PressureStatus;

    /// Handles pressure conditions.
    async fn handle_pressure(&mut self, action: PressureAction) -> Result<(), Error>;

    /// Sets thresholds for pressure levels.
    fn set_pressure_thresholds(&mut self, thresholds: PressureThresholds) -> Result<(), Error>;
}

/// Trait for validating configurations.
pub trait Validate {
    /// Validates the configuration.
    fn validate(&self) -> ValidationResult;
}

#[async_trait]
pub trait ResourceManager: Send + Sync {
    async fn acquire_resources(
        &mut self,
        requirements: ResourceRequirements,
    ) -> Result<ResourceHandle, Error>;
    async fn release_resources(&mut self, handle: ResourceHandle) -> Result<(), Error>;
    fn resource_usage(&self) -> ResourceUsage;
    fn set_resource_limits(&mut self, limits: ResourceLimits) -> Result<(), Error>;
}

#[async_trait]
pub trait Cleanup: Send + Sync {
    async fn cleanup(&mut self) -> Result<(), Error>;
    async fn emergency_cleanup(&mut self) -> Result<(), Error>;
    fn register_cleanup_handler(&mut self, handler: Box<dyn CleanupHandler>);
    fn cleanup_status(&self) -> CleanupStatus;
}

pub trait CleanupHandler: Send + Sync {
    fn handle_cleanup(&self) -> Result<(), Error>;
    fn cleanup_priority(&self) -> CleanupPriority;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CleanupPriority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone)]
pub struct CleanupError {
    pub message: String,
    pub code: u32,
}

pub struct CleanupStatus {
    pub pending_cleanups: usize,
    pub last_cleanup: Option<u64>,
    pub failed_cleanups: Vec<CleanupError>,
}

#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub memory_mb: u64,
    pub cpu_cores: f32,
    pub storage_gb: u64,
    pub network_mbps: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceHandle {
    pub id: String,
    pub allocated: ResourceAllocation,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub memory_mb: u64,
    pub cpu_cores: f32,
    pub storage_gb: u64,
    pub network_mbps: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub allocated: ResourceAllocation,
    pub available: ResourceAllocation,
    pub utilization: ResourceUtilization,
}

#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub memory_mb: u64,
    pub cpu_cores: f32,
    pub storage_gb: u64,
    pub network_mbps: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    pub memory_percent: f32,
    pub cpu_percent: f32,
    pub storage_percent: f32,
    pub network_percent: f32,
}

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn acquire_permit(&mut self) -> Result<(), Error>;
    async fn acquire_n_permits(&mut self, n: u32) -> Result<(), Error>;
    fn set_rate_limit(&mut self, permits_per_second: u32) -> Result<(), Error>;
    fn current_rate(&self) -> u32;
}

#[async_trait]
pub trait BackpressureControl: Send + Sync {
    async fn apply_backpressure(&mut self, level: PressureLevel) -> Result<(), Error>;
    async fn release_backpressure(&mut self) -> Result<(), Error>;
    fn backpressure_status(&self) -> BackpressureStatus;
    fn set_backpressure_thresholds(
        &mut self,
        thresholds: BackpressureThresholds,
    ) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct BackpressureStatus {
    pub active: bool,
    pub level: PressureLevel,
    pub duration: Duration,
    pub cause: String,
}

#[derive(Debug, Clone)]
pub struct BackpressureThresholds {
    pub soft_limit: f32,
    pub hard_limit: f32,
    pub recovery_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct Version(pub u64);
