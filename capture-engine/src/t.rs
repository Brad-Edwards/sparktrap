// Core trait definitions for managers and components
mod traits {
    use error::Error;

    /// Common trait for all managers, providing lifecycle methods
    pub trait Manager: Send + Sync {
        fn init(&mut self) -> Result<(), Error>;  // Add initialization phase
        fn start(&mut self) -> Result<(), Error>;
        fn pause(&mut self) -> Result<(), Error>; // Add pause capability
        fn resume(&mut self) -> Result<(), Error>; // Add resume capability
        fn stop(&mut self) -> Result<(), Error>;
        fn shutdown(&mut self) -> Result<(), Error>; // Add clean shutdown
        fn health_check(&self) -> HealthStatus;
    }

    /// Trait for event handling in an event-driven architecture
    pub trait EventHandler<E>: Send + Sync {
        /// Handles an incoming event
        fn handle_event(&mut self, event: E);
    }

    /// Trait for processing packets
    pub trait PacketProcessor {
        fn process_packet(&mut self, packet: &mut Packet) -> Result<(), Error>;
        fn process_batch(&mut self, packets: &mut [Packet]) -> Result<(), Error>;
    }

    pub enum HealthStatus {
        Healthy,
        Degraded,
        Unhealthy,
    }

    // Placeholder for Packet type
    pub struct Packet {
        pub timestamp: u64,
        pub data: Vec<u8>,
        pub length: usize,
        pub metadata: PacketMetadata,
        pub buffer_id: Option<BufferId>, // For zero-copy operations
    }

    /// Separate pressure awareness trait
    pub trait PressureAware {
        fn get_pressure(&self) -> PressureStatus;
        fn handle_pressure(&mut self, action: PressureAction) -> Result<(), Error>;
        fn set_pressure_thresholds(&mut self, thresholds: PressureThresholds) -> Result<(), Error>;
    }

    /// Configuration validation trait
    pub trait Validate {
        fn validate(&self) -> ValidationResult;
    }
}

// Manages memory allocation and lifecycle <source_id data="capture_engine_module_architecture.md" />
mod buffer_manager {
    use traits::{Manager, EventHandler};
    use error::Error;

    /// Events specific to BufferManager 
    pub enum BufferEvent {
        MemoryPressure(PressureLevel),    // Memory pressure changed
        BufferReleased,                   // Buffer returned to pool
        PoolExhausted,                    // No buffers available
        WatermarkCrossed(WatermarkType),  // Watermark threshold crossed
    }

    pub enum WatermarkType {
        High,
        Low,
        Critical
    }

    /// BufferManager handles packet memory pools and buffer allocation
    pub trait BufferManager: Manager + EventHandler<BufferEvent> + PressureAware{
        /// Allocates a managed buffer for packet storage
        fn acquire_buffer(&mut self) -> Result<ManagedBuffer, Error>;
        
        /// Gets current memory pressure status
        fn get_memory_pressure(&self) -> PressureStatus;
        
        /// Sets pressure watermarks for event generation
        fn set_pressure_watermarks(&mut self, low: f32, high: f32) -> Result<(), Error>;
        
        /// Handles pressure conditions
        fn handle_pressure(&mut self, action: PressureAction) -> Result<(), Error>;

        fn acquire_zero_copy_buffer(&mut self) -> Result<ZeroCopyBuffer, Error>;
        fn release_buffer(&mut self, buffer: BufferId) -> Result<(), Error>;
    }

    /// RAII wrapper for buffer lifecycle
    pub trait ManagedBuffer: Send + Sync {
        fn as_slice(&self) -> &[u8];
        fn as_slice_mut(&mut self) -> &mut [u8];
        fn metadata(&self) -> &BufferMetadata;
    }

    pub enum PressureAction {
        DropOldest,
        ThrottleInput,
        ForceFlush,
        EmergencyRelease,
    }

    pub struct PressureStatus {
        pub level: PressureLevel,
        pub utilization: f32,
        pub available_buffers: usize,
    }

    pub enum PressureLevel {
        Normal,
        Elevated,
        Critical,
        Overflow,
    }
}

// Core capture pipeline orchestration <source_id data="capture_engine_module_architecture.md" />
mod capture_manager {
    use traits::{Manager, PacketProcessor};
    use error::Error;

    /// CaptureManager coordinates the primary data pipeline
    pub trait CaptureManager: Manager + PacketProcessor {
        /// Receives a packet from the InterfaceManager
        fn receive_packet(&mut self, packet: &mut Packet) -> Result<(), Error>;
        
        fn get_pipeline_pressure(&self) -> Vec<(Stage, PressureStatus)>;
        
        fn apply_stage_backpressure(&mut self, stage: Stage, action: PressureAction) -> Result<(), Error>;

        fn receive_batch(&mut self, packets: &mut [Packet]) -> Result<(), Error>;
        fn set_batch_size(&mut self, size: usize) -> Result<(), Error>;
    }

    pub enum Stage {
        Ingestion,
        LightParse,
        DeepParse,
        Filtering,
        Output,
    }
}

// Cloud integration and lifecycle event handling <source_id data="capture_engine_module_architecture.md" />
mod cloud_manager {
    use traits::{Manager, EventHandler};
    use error::Error;
    use super::config_manager::configs;

    /// Events specific to CloudManager
    pub enum CloudEvent {
        LifecycleEvent(CloudLifecycleEvent),
        NetworkEvent(NetworkEvent),
        ResourceEvent(ResourceEvent),
    }

    /// Cloud lifecycle events
    pub enum CloudLifecycleEvent {
        InstanceStart(configs::InstanceMetadata),
        InstanceStop,
        InstanceTerminate,
        InstancePreempt(Duration),
    }

    /// Network-related events
    pub enum NetworkEvent {
        MirrorSessionUpdate(configs::MirrorSessionConfig),
        InterfaceChange(configs::NetworkInterfaceUpdate),
        BandwidthChange(configs::BandwidthLimit),
    }

    /// Resource-related events
    pub enum ResourceEvent {
        MemoryLimit(u64),
        CpuLimit(u32),
        StorageLimit(u64),
    }

    /// CloudManager handles instance-level cloud integration
    pub trait CloudManager: Manager + EventHandler<CloudEvent> {
        fn get_instance_metadata(&self) -> Result<configs::InstanceMetadata, Error>;
        fn get_network_config(&self) -> Result<configs::NetworkConfig, Error>;
        fn configure_mirror_session(&mut self, config: configs::MirrorSessionConfig) -> Result<(), Error>;
        fn validate_mirror_session(&self) -> Result<configs::MirrorSessionStatus, Error>;
        fn get_instance_limits(&self) -> Result<configs::InstanceLimits, Error>;
        fn get_network_capabilities(&self) -> Result<configs::NetworkCapabilities, Error>;
    }
}

// Control plane communication <source_id data="capture_engine_module_architecture.md" />
mod control_manager {
    use traits::{Manager, EventHandler};
    use error::Error;

    /// Events specific to ControlManager
    pub enum ControlEvent {
        ConfigurationUpdate(Configuration),
        Command(ControlCommand),
    }

    /// Control commands from the control plane
    pub enum ControlCommand {
        StartCapture,
        StopCapture,
        UpdateFilters,
    }

    /// Configuration data
    pub struct Configuration {
        // Configuration fields
    }

    /// ControlManager manages communication with the control plane
    pub trait ControlManager: Manager + EventHandler<ControlEvent> {
        /// Sends status updates to the control plane
        fn send_status(&self) -> Result<(), Error>;
    }
}

// Packet filtering <source_id data="capture_engine_module_architecture.md" />
mod filter_manager {
    use traits::{Manager, PacketProcessor, EventHandler, PressureAware};
    use error::Error;
    use std::time::Duration;

    // Filter-specific events
    pub enum FilterEvent {
        RulesetUpdate(FilterRulesetId),
        OffloadStatusChange(OffloadStatus),
        FilterPressure(PressureLevel),
        RuleMatchThreshold(RuleMatchInfo),
    }

    // Core filter manager interface
    pub trait FilterManager: Manager + PacketProcessor + EventHandler<FilterEvent> + PressureAware {
        // Rule Management
        fn update_rules(&mut self, rules: FilterRuleset) -> Result<(), Error>;
        fn get_active_ruleset(&self) -> Result<FilterRulesetId, Error>;
        fn validate_ruleset(&self, rules: &FilterRuleset) -> Result<ValidationResult, Error>;
        
        // Hardware Offload
        fn configure_hw_filters(&mut self, rules: &FilterRuleset) -> Result<OffloadStatus, Error>;
        fn get_offload_capabilities(&self) -> HardwareCapabilities;
        
        // Runtime Operations
        fn get_filter_stats(&self) -> Result<FilterStats, Error>;
        fn clear_stats(&mut self) -> Result<(), Error>;
        
        // Performance Management
        fn set_batch_parameters(&mut self, params: BatchParameters) -> Result<(), Error>;
        fn get_performance_metrics(&self) -> Result<FilterMetrics, Error>;
    }

    // Supporting types
    pub struct FilterRuleset {
        pub id: FilterRulesetId,
        pub rules: Vec<FilterRule>,
        pub default_action: FilterAction,
        pub optimization_hints: Vec<OptimizationHint>,
        pub hw_offload_preference: OffloadPreference,
    }

    pub struct FilterRule {
        pub id: RuleId,
        pub priority: i32,
        pub conditions: Vec<FilterCondition>,
        pub action: FilterAction,
        pub metadata: RuleMetadata,
    }

    pub enum FilterCondition {
        // Layer 2
        EtherType(u16),
        MacAddress { src: Option<MacAddr>, dst: Option<MacAddr> },
        VlanId(u16),

        // Layer 3
        IpProtocol(u8),
        IpAddress { src: Option<IpNetwork>, dst: Option<IpNetwork> },
        IpFlags(u8),
        
        // Layer 4
        Port { src: Option<PortRange>, dst: Option<PortRange> },
        TcpFlags(u8),
        
        // Compound
        And(Vec<FilterCondition>),
        Or(Vec<FilterCondition>),
        Not(Box<FilterCondition>),
    }

    pub enum FilterAction {
        Accept,
        Drop,
        Sample(f32),
        Mirror { target: String },
        Modify(PacketModification),
    }

    pub struct BatchParameters {
        pub max_batch_size: usize,
        pub timeout: Duration,
        pub optimization_level: OptimizationLevel,
    }

    pub struct FilterStats {
        pub total_packets: u64,
        pub matched_packets: u64,
        pub dropped_packets: u64,
        pub rule_matches: HashMap<RuleId, u64>,
        pub performance_metrics: FilterMetrics,
    }

    pub struct FilterMetrics {
        pub average_latency: Duration,
        pub max_latency: Duration,
        pub packets_per_second: u64,
        pub current_pressure: PressureLevel,
        pub hw_offload_ratio: f32,
    }

    pub enum OptimizationHint {
        PreferHardware,
        PreferSoftware,
        OptimizeForLatency,
        OptimizeForThroughput,
        ExpectedRate(u64),
    }

    pub struct HardwareCapabilities {
        pub max_rules: usize,
        pub supported_conditions: Vec<FilterConditionType>,
        pub supported_actions: Vec<FilterActionType>,
        pub performance_limits: HardwareLimits,
    }

    pub enum OffloadStatus {
        Full,
        Partial { software_rules: Vec<RuleId> },
        None { reason: String },
    }
}

// Network interface management <source_id data="capture_engine_module_architecture.md" />
mod interface_manager {
    use traits::{Manager, EventHandler, PressureAware};
    use error::Error;

    pub enum InterfaceEvent {
        MirrorSessionUpdate(MirrorSessionStatus),
        InterfaceStateChange(InterfaceState),
        NetworkPerformanceChange(NetworkPerformanceInfo),
        PacketDropEvent(PacketDropInfo),
        BufferExhaustion(BufferStatus),
    }

    pub trait InterfaceManager: Manager + EventHandler<InterfaceEvent> + PressureAware {
        // Core Interface Management
        fn initialize(&mut self, config: &InterfaceConfig) -> Result<(), Error>;
        fn shutdown(&mut self) -> Result<(), Error>;
        
        // Packet Reception
        fn capture_packets(&mut self) -> Result<Vec<Packet>, Error>;
        fn capture_batch(&mut self, batch_size: usize) -> Result<PacketBatch, Error>;
        
        // Mirror Session Management
        fn configure_mirror_session(&mut self, config: MirrorSessionConfig) -> Result<(), Error>;
        fn get_mirror_session_status(&self) -> Result<MirrorSessionStatus, Error>;
        fn validate_mirror_config(&self, config: &MirrorSessionConfig) -> Result<ValidationResult, Error>;

        // ENI Management
        fn attach_eni(&mut self, eni_config: ENIConfig) -> Result<(), Error>;
        fn detach_eni(&mut self, eni_id: &str) -> Result<(), Error>;
        fn get_eni_status(&self, eni_id: &str) -> Result<ENIStatus, Error>;

        // Performance Management
        fn set_capture_rate(&mut self, rate: CaptureRate) -> Result<(), Error>;
        fn get_input_pressure(&self) -> PressureStatus;
        fn pause_capture(&mut self) -> Result<(), Error>;
        fn resume_capture(&mut self) -> Result<(), Error>;

        // Hardware Offload Configuration
        fn configure_dpdk(&mut self, config: DPDKConfig) -> Result<(), Error>;
        fn configure_xdp(&mut self, config: XDPConfig) -> Result<(), Error>;
        
        // Monitoring and Statistics
        fn get_rx_stats(&self) -> Result<RxStats, Error>;
        fn get_mirror_stats(&self) -> Result<MirrorStats, Error>;
        fn get_interface_metrics(&self) -> Result<InterfaceMetrics, Error>;
    }

    // Status and Metrics Types
    pub struct MirrorSessionStatus {
        pub session_id: String,
        pub state: SessionState,
        pub packets_received: u64,
        pub packets_dropped: u64,
        pub last_error: Option<String>,
    }

    pub struct ENIStatus {
        pub eni_id: String,
        pub state: ENIState,
        pub link_status: LinkStatus,
        pub bandwidth: BandwidthInfo,
        pub error_counts: ErrorCounts,
    }

    pub struct InterfaceMetrics {
        pub timestamp: u64,
        pub rx_packets: u64,
        pub rx_bytes: u64,
        pub rx_errors: u64,
        pub dropped_packets: u64,
        pub buffer_utilization: f32,
        pub bandwidth_utilization: f32,
    }

    // Performance Types
    pub struct CaptureRate {
        pub packets_per_second: Option<u32>,
        pub bytes_per_second: Option<u64>,
        pub burst_size: Option<u32>,
    }

    pub struct OffloadConfig {
        pub dpdk_enabled: bool,
        pub xdp_enabled: bool,
        pub rx_queues: u16,
        pub tx_queues: u16,
        pub queue_size: u16,
        pub offload_features: Vec<OffloadFeature>,
    }

    // Supporting Types
    #[derive(Debug)]
    pub enum SessionState {
        Initializing,
        Active,
        Error(String),
        Stopping,
        Stopped,
    }

    #[derive(Debug)]
    pub enum ENIState {
        Attaching,
        Active,
        Detaching,
        Detached,
        Error(String),
    }

    #[derive(Debug)]
    pub enum ENIType {
        Primary,
        Mirror,
        Trunk,
    }
}

// Data output handling <source_id data="capture_engine_module_architecture.md" />
mod output_manager {
    use traits::{Manager, EventHandler, PressureAware};
    use error::Error;

    pub enum OutputEvent {
        DestinationStatus(DestinationStatus),
        BufferThreshold(BufferThresholdEvent),
        RotationTriggered(RotationTrigger),
        WriteError(WriteFailure),
        BackpressureEvent(BackpressureStatus),
    }

    /// OutputManager handles high-throughput data output
    pub trait OutputManager: Manager + EventHandler<OutputEvent> + PressureAware {
        // Core Output Operations
        fn send_data(&mut self, data: &OutputData) -> Result<(), Error>;
        fn send_batch(&mut self, batch: &OutputBatch) -> Result<(), Error>;
        
        // Flush Control
        fn flush(&mut self, mode: FlushMode) -> Result<(), Error>;
        fn flush_destination(&mut self, destination_id: &str, mode: FlushMode) -> Result<(), Error>;

        // Destination Management
        fn add_destination(&mut self, config: &configs::OutputDestinationConfig) -> Result<(), Error>;
        fn remove_destination(&mut self, destination_id: &str) -> Result<(), Error>;
        fn get_destination_status(&self, destination_id: &str) -> Result<DestinationStatus, Error>;

        // Performance Management
        fn get_output_pressure(&self) -> PressureStatus;
        fn propagate_pressure(&mut self, pressure: PressureStatus) -> Result<(), Error>;
        fn set_write_qos(&mut self, qos: QualityOfService) -> Result<(), Error>;

        // Monitoring
        fn get_output_stats(&self) -> Result<OutputStats, Error>;
        fn get_write_metrics(&self) -> Result<WriteMetrics, Error>;
    }

    // Core Data Types
    pub struct OutputData {
        pub timestamp: u64,
        pub data: Vec<u8>,
        pub metadata: OutputMetadata,
        pub routing: Option<DestinationRouting>,
    }

    pub struct OutputBatch {
        pub packets: Vec<OutputData>,
        pub batch_metadata: BatchMetadata,
        pub write_options: WriteOptions,
    }

    pub enum FlushMode {
        Normal,          // Regular flush
        Forced,          // Flush immediately
        Emergency,       // Emergency flush during resource pressure
        Selective(Vec<String>), // Flush specific destinations
    }

    pub struct DestinationStatus {
        pub id: String,
        pub state: DestinationState,
        pub current_pressure: PressureStatus,
        pub write_stats: WriteStats,
        pub last_error: Option<String>,
    }

    pub enum DestinationState {
        Active,
        Degraded(String),
        Blocked(String),
        Failed(String),
        Recovering,
    }

    pub struct WriteStats {
        pub bytes_written: u64,
        pub packets_written: u64,
        pub write_errors: u64,
        pub current_throughput: f64,
        pub average_latency: Duration,
    }

    pub struct WriteMetrics {
        pub timestamp: u64,
        pub queue_depth: usize,
        pub buffer_usage: f64,
        pub write_throughput: f64,
        pub error_rate: f64,
        pub destination_metrics: HashMap<String, DestinationMetrics>,
    }

    pub struct QualityOfService {
        pub priority: Priority,
        pub max_latency: Duration,
        pub bandwidth_limit: Option<u64>,
        pub retry_policy: RetryPolicy,
    }

    pub enum Priority {
        Realtime,
        High,
        Normal,
        Background,
    }

    pub struct RetryPolicy {
        pub max_retries: u32,
        pub retry_interval: Duration,
        pub backoff_multiplier: f32,
    }
}

// Protocol analysis <source_id data="capture_engine_module_architecture.md" />
mod protocol_manager {
    use traits::{Manager, PacketProcessor};
    use error::Error;

    /// Header information extracted from a packet
    pub struct HeaderInfo {
        // Header fields
    }

    /// Deep inspection result
    pub struct InspectionResult {
        // Inspection result fields
    }

    /// ProtocolManager handles protocol analysis at two levels
    pub trait ProtocolManager: Manager + PacketProcessor {
        /// Performs header parsing on a packet
        fn parse_headers(&mut self, packet: &mut Packet) -> Result<HeaderInfo, Error>;

        /// Performs deep inspection on a packet
        fn deep_inspect(&mut self, packet: &mut Packet) -> Result<InspectionResult, Error>;
    }
}

// Security management <source_id data="capture_engine_module_architecture.md" />
mod security_manager {
    use traits::{Manager, EventHandler};
    use error::Error;
    use std::time::Duration;

    pub enum SecurityEvent {
        AuthenticationAttempt(AuthAttempt),
        AuthorizationRequest(AuthRequest),
        PolicyUpdate(PolicyUpdate),
        CertificateEvent(CertificateInfo),
    }

    pub trait SecurityManager: Manager + EventHandler<SecurityEvent> {
        // Authentication for control plane communication
        fn authenticate_request(&self, request: &AuthRequest) -> Result<AuthToken, Error>;
        fn validate_token(&self, token: &AuthToken) -> Result<ValidationResult, Error>;
        fn revoke_token(&mut self, token: &AuthToken) -> Result<(), Error>;

        // Authorization for operations
        fn authorize_action(&self, action: &Action, context: &SecurityContext) -> Result<AuthzDecision, Error>;
        fn check_permissions(&self, subject: &Subject, resource: &Resource) -> Result<Vec<Permission>, Error>;

        // Crypto for securing captured data
        fn encrypt_data(&self, data: &[u8], context: &CryptoContext) -> Result<Vec<u8>, Error>;
        fn decrypt_data(&self, data: &[u8], context: &CryptoContext) -> Result<Vec<u8>, Error>;

        // Certificate management for TLS
        fn rotate_certificates(&mut self) -> Result<(), Error>;
        fn validate_certificate(&self, cert: &Certificate) -> Result<ValidationResult, Error>;
    }

    // Supporting types focused on capture engine needs
    pub struct SecurityContext {
        pub timestamp: u64,
        pub subject: Subject,
        pub resource: Resource,
        pub trace_id: Option<String>,
    }

    pub struct Subject {
        pub id: String,
        pub roles: Vec<Role>,
        pub attributes: HashMap<String, String>,
    }

    pub struct Resource {
        pub type_: ResourceType,
        pub id: String,
        pub attributes: HashMap<String, String>,
    }
}

// State coordination using an event-driven approach <source_id data="capture_engine_module_architecture.md" />
mod state_manager {
    use traits::{Manager, EventHandler};
    use error::Error;
    use std::collections::HashMap;

    pub enum StateEvent {
        StateChange(State),
        ComponentStateChange(ComponentState),
        PressureStateChange(PressureState),
        TransitionEvent(StateTransition),
        RecoveryEvent(RecoveryAction),
    }

    pub trait StateManager: Manager + EventHandler<StateEvent> {
        // Core State Management
        fn get_state(&self) -> Result<State, Error>;
        fn get_component_state(&self, component: &str) -> Result<ComponentState, Error>;
        fn persist_state(&self) -> Result<(), Error>;
        
        // Pressure Management
        fn update_pressure_state(&mut self, pressure: PressureState) -> Result<(), Error>;
        fn enforce_pressure_policy(&mut self, policy: PressurePolicy) -> Result<(), Error>;
        
        // State Transitions
        fn request_transition(&mut self, transition: StateTransition) -> Result<(), Error>;
        fn validate_transition(&self, transition: &StateTransition) -> Result<bool, Error>;
        
        // Recovery Management
        fn initiate_recovery(&mut self, action: RecoveryAction) -> Result<(), Error>;
        fn get_recovery_status(&self) -> Result<RecoveryStatus, Error>;
    }

    pub struct State {
        pub capture_state: CaptureState,
        pub components: HashMap<String, ComponentState>,
        pub pressure_state: PressureState,
        pub recovery_state: Option<RecoveryState>,
    }

    pub enum CaptureState {
        Initializing,
        Ready,
        Capturing,
        Paused,
        ShuttingDown,
        Error(String),
    }

    pub struct ComponentState {
        pub status: ComponentStatus,
        pub health: HealthStatus,
        pub last_update: u64,
        pub error_count: u32,
    }

    pub enum ComponentStatus {
        Starting,
        Running,
        Degraded(String),
        Failed(String),
        Recovering,
        Stopped,
    }

    pub struct PressureState {
        pub memory: PressureLevel,
        pub cpu: PressureLevel,
        pub network: PressureLevel,
        pub storage: PressureLevel,
        pub component_pressure: HashMap<String, PressureLevel>,
    }

    pub struct StateTransition {
        pub from_state: CaptureState,
        pub to_state: CaptureState,
        pub reason: String,
        pub required_components: Vec<String>,
    }

    pub struct RecoveryAction {
        pub component: String,
        pub action_type: RecoveryType,
        pub max_attempts: u32,
        pub backoff_policy: BackoffPolicy,
    }

    pub enum RecoveryType {
        Restart,
        Reinitialize,
        Failover,
        GracefulDegrade,
    }
}

// Local storage management <source_id data="capture_engine_module_architecture.md" />
mod storage_manager {
    use traits::{Manager, EventHandler, PressureAware};
    use error::Error;
    use std::path::PathBuf;

    pub enum StorageEvent {
        SpaceThreshold(SpaceThresholdEvent),
        WriteFailure(WriteFailureInfo),
        DeviceError(DeviceError),
        PerformanceDegraded(PerformanceInfo),
    }

    pub trait StorageManager: Manager + EventHandler<StorageEvent> + PressureAware {
        // Core Storage Operations
        fn write(&mut self, data: &StorageData) -> Result<StorageId, Error>;
        fn write_batch(&mut self, batch: &[StorageData]) -> Result<Vec<StorageId>, Error>;
        fn read(&mut self, id: &StorageId) -> Result<StorageData, Error>;
        fn delete(&mut self, id: &StorageId) -> Result<(), Error>;

        // Space Management
        fn get_storage_pressure(&self) -> PressureStatus;
        fn recover_space(&mut self, required: usize) -> Result<(), Error>;
        fn get_space_stats(&self) -> Result<SpaceStats, Error>;

        // Performance Management
        fn flush(&mut self, mode: FlushMode) -> Result<(), Error>;
        fn get_device_metrics(&self) -> Result<DeviceMetrics, Error>;
    }

    pub struct DeviceMetrics {
        pub read_iops: u64,
        pub write_iops: u64,
        pub read_throughput: u64,
        pub write_throughput: u64,
        pub latency_us: u64,
        pub queue_depth: u32,
        pub utilization: f32,
    }

    pub struct SpaceStats {
        pub total_space: u64,
        pub used_space: u64,
        pub available_space: u64,
        pub write_buffer_usage: f32,
    }

    pub enum FlushMode {
        Normal,
        Emergency,
    }
}

// Metrics, logging, and monitoring interfaces
mod telemetry_manager {
    use traits::{Manager};
    use error::Error;
    use std::collections::HashMap;

    // Align with OpenTelemetry semantic conventions
    pub struct TelemetryData {
        pub timestamp: u64,
        pub name: String,                     // OpenTelemetry metric name
        pub description: Option<String>,      // Optional description
        pub unit: Option<MetricUnit>,         // Standardized units
        pub metric_type: MetricType,
        pub value: MetricValue,
        pub attributes: HashMap<String, String>, // OpenTelemetry calls these attributes
        pub resource: Option<HashMap<String, String>>, // Resource attributes (e.g., host.name, service.name)
    }

    // Standard OpenTelemetry metric types
    #[derive(Debug, Clone)]
    pub enum MetricType {
        Counter,      // Monotonic counter
        UpDownCounter,// Non-monotonic counter
        Gauge,        // Point-in-time measurement
        Histogram,    // Distribution of values
    }

    // Standard units following OpenTelemetry conventions
    #[derive(Debug, Clone)]
    pub enum MetricUnit {
        // Time
        Nanoseconds,
        Microseconds,
        Milliseconds,
        Seconds,
        // Bytes
        Bytes,
        KiloBytes,
        MegaBytes,
        // Rates
        BytesPerSecond,
        PacketsPerSecond,
        // Dimensionless
        Percent,
        Count,
    }

    #[derive(Debug, Clone)]
    pub enum MetricValue {
        Int(i64),
        Float(f64),
        // Histogram data aligned with OpenTelemetry
        Histogram {
            count: u64,
            sum: f64,
            buckets: Vec<(f64, u64)>, // (boundary, count)
        },
    }

    // System pressure monitoring
    #[derive(Debug)]
    pub struct PressureMetrics {
        pub timestamp: u64,
        pub memory_pressure: f32,
        pub cpu_pressure: f32,
        pub io_pressure: f32,
        // Additional context for pressure metrics
        pub attributes: HashMap<String, String>,
    }

    #[derive(Debug)]
    pub enum PressureCondition {
        MemoryHigh { usage_percent: f32 },
        CpuHigh { usage_percent: f32 },
        IoSaturated { wait_time_ms: u64 },
        ResourceExhausted { resource: String, context: HashMap<String, String> },
    }

    // Export formats supporting standard protocols
    pub enum ExportFormat {
        OpenTelemetry,
        Prometheus,
        JSON,
    }

    /// TelemetryManager collects metrics following OpenTelemetry standards
    pub trait TelemetryManager: Manager {
        // Core collection
        fn collect(&mut self, data: TelemetryData) -> Result<(), Error>;
        
        // Pressure monitoring
        fn record_pressure_metrics(&mut self, metrics: PressureMetrics) -> Result<(), Error>;
        fn alert_pressure_condition(&mut self, condition: PressureCondition) -> Result<(), Error>;
        
        // Reporting
        fn report_metrics(&self) -> Result<(), Error>;
        
        // Export in standard formats
        fn export_metrics(&self, format: ExportFormat) -> Result<Vec<u8>, Error>;
    }

    // Common metric names following OpenTelemetry semantic conventions
    pub mod metric_names {
        pub const MEMORY_USAGE: &str = "process.runtime.memory";
        pub const CPU_USAGE: &str = "process.runtime.cpu";
        pub const PACKETS_RECEIVED: &str = "network.packets.received";
        pub const PACKETS_DROPPED: &str = "network.packets.dropped";
        pub const BUFFER_USAGE: &str = "memory.buffer.usage";
        pub const PROCESSING_LATENCY: &str = "processing.latency";
    }

    // Common attribute keys following OpenTelemetry semantic conventions
    pub mod attribute_keys {
        // Resource attributes
        pub const SERVICE_NAME: &str = "service.name";
        pub const SERVICE_VERSION: &str = "service.version";
        pub const HOST_NAME: &str = "host.name";
        
        // Network attributes
        pub const NET_INTERFACE: &str = "net.interface.name";
        pub const NET_PROTOCOL: &str = "net.protocol.name";
        
        // Component attributes
        pub const COMPONENT: &str = "component";
        pub const ERROR_TYPE: &str = "error.type";
    }
}

// Centralized event system for inter-manager communication
mod event_system {
    use std::sync::Arc;
    use tokio::sync::{mpsc, broadcast};
    use std::time::Duration;

    // Core Event Types
    pub enum SystemEvent {
        // Component Events
        Buffer(BufferEvent),
        Capture(CaptureEvent),
        Cloud(CloudEvent),
        Control(ControlEvent),
        Filter(FilterEvent),
        Interface(InterfaceEvent),
        Output(OutputEvent),
        Protocol(ProtocolEvent),
        Security(SecurityEvent),
        State(StateEvent),
        Storage(StorageEvent),
        Telemetry(TelemetryEvent),

        // System-wide Events
        Pressure(PressureEvent),
        Resource(ResourceEvent),
        Lifecycle(LifecycleEvent),
        Error(ErrorEvent),
    }

    // Event Metadata
    pub struct EventMetadata {
        pub id: EventId,
        pub timestamp: u64,
        pub priority: EventPriority,
        pub correlation_id: Option<String>,
        pub source: EventSource,
        pub trace_context: Option<TraceContext>,
    }

    // Event Wrapper with Metadata
    pub struct Event {
        pub metadata: EventMetadata,
        pub payload: SystemEvent,
    }

    // Event Priority Levels
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EventPriority {
        Critical,
        High,
        Normal,
        Low,
        Background,
    }

    // Event Source Information
    pub struct EventSource {
        pub component: String,
        pub instance_id: String,
        pub location: String,
    }

    // Event Subscription
    pub struct EventSubscription {
        pub filters: Vec<EventFilter>,
        pub buffer_size: usize,
        pub timeout: Duration,
    }

    // Event Filtering
    pub enum EventFilter {
        ByType(Vec<SystemEvent>),
        ByPriority(EventPriority),
        BySource(String),
        ByCorrelation(String),
        Custom(Box<dyn Fn(&Event) -> bool + Send + Sync>),
    }

    // Core Event System
    pub struct EventSystem {
        // Channels for different priority levels
        critical_tx: mpsc::Sender<Event>,
        high_tx: mpsc::Sender<Event>,
        normal_tx: mpsc::Sender<Event>,
        low_tx: mpsc::Sender<Event>,
        
        // Broadcast channel for subscribers
        broadcast_tx: broadcast::Sender<Event>,
        
        // Event correlation
        correlation_tracker: Arc<CorrelationTracker>,
        
        // Backpressure handling
        pressure_controller: Arc<PressureController>,
    }

    impl EventSystem {
        // Core Operations
        pub async fn publish(&self, event: Event) -> Result<(), EventError> {
            self.handle_backpressure(&event).await?;
            self.route_event(event).await
        }

        pub async fn subscribe(&self, subscription: EventSubscription) 
            -> Result<EventSubscriber, EventError> {
            // Implementation
        }

        // Event Routing
        async fn route_event(&self, event: Event) -> Result<(), EventError> {
            match event.metadata.priority {
                EventPriority::Critical => self.critical_tx.send(event).await?,
                EventPriority::High => self.high_tx.send(event).await?,
                EventPriority::Normal => self.normal_tx.send(event).await?,
                EventPriority::Low => self.low_tx.send(event).await?,
                EventPriority::Background => {
                    if !self.pressure_controller.is_pressured() {
                        self.low_tx.send(event).await?
                    }
                }
            }
            Ok(())
        }

        // Backpressure Management
        async fn handle_backpressure(&self, event: &Event) -> Result<(), EventError> {
            if self.pressure_controller.is_pressured() {
                match event.metadata.priority {
                    EventPriority::Critical | EventPriority::High => Ok(()),
                    _ => {
                        self.pressure_controller.wait_for_capacity().await?;
                        Ok(())
                    }
                }
            } else {
                Ok(())
            }
        }
    }

    // Event Subscriber
    pub struct EventSubscriber {
        rx: mpsc::Receiver<Event>,
        filters: Vec<EventFilter>,
    }

    impl EventSubscriber {
        pub async fn next(&mut self) -> Option<Event> {
            while let Some(event) = self.rx.recv().await {
                if self.matches_filters(&event) {
                    return Some(event);
                }
            }
            None
        }

        fn matches_filters(&self, event: &Event) -> bool {
            self.filters.iter().all(|filter| match filter {
                EventFilter::ByType(types) => types.contains(&event.payload),
                EventFilter::ByPriority(priority) => &event.metadata.priority >= priority,
                EventFilter::BySource(source) => event.metadata.source.component == *source,
                EventFilter::ByCorrelation(id) => event.metadata.correlation_id.as_ref() == Some(id),
                EventFilter::Custom(f) => f(event),
            })
        }
    }
}

mod config_manager {
    use std::time::Duration;
    use std::net::IpAddr;
    use std::path::PathBuf;
    use super::traits::Validate;

    // Common types used across multiple configs
    pub mod common {
        use super::*;

        #[derive(Debug, Clone)]
        pub enum Protocol {
            TCP,
            UDP,
            ICMP,
            SCTP,
            Custom(String),
        }

        #[derive(Debug, Clone)]
        pub enum LogLevel {
            Error,
            Warn,
            Info,
            Debug,
            Trace,
        }

        #[derive(Debug, Clone)]
        pub struct TlsConfig {
            pub cert_path: PathBuf,
            pub key_path: PathBuf,
            pub ca_path: Option<PathBuf>,
            pub verify_peer: bool,
        }

        #[derive(Debug, Clone)]
        pub struct PressureThresholds {
            pub memory: f32,
            pub cpu: f32,
            pub storage: f32,
            pub network: f32,
        }

        #[derive(Debug, Clone)]
        pub enum UpdateStrategy {
            Immediate,
            Graceful,
            Scheduled(Duration),
        }

        #[derive(Debug, Clone)]
        pub struct CompressionConfig {
            pub algorithm: CompressionAlgorithm,
            pub level: u8,
        }

        #[derive(Debug, Clone)]
        pub enum CompressionAlgorithm {
            Gzip,
            Lz4,
            Zstd,
            None,
        }

        #[derive(Debug, Clone)]
        pub struct QualityOfService {
            pub priority: Priority,
            pub max_latency: Duration,
            pub bandwidth_limit: Option<u64>,
        }

        #[derive(Debug, Clone)]
        pub enum Priority {
            Realtime,
            High,
            Normal,
            Background,
        }

        #[derive(Debug, Clone)]
        pub struct RetryPolicy {
            pub max_retries: u32,
            pub retry_interval: Duration,
            pub backoff_multiplier: f32,
        }
    }

    // Main configuration types for each manager
    pub mod configs {
        use super::*;
        use super::common::*;

        // Buffer Manager Configuration
        #[derive(Debug, Clone)]
        pub struct BufferConfig {
            pub pool_size: usize,
            pub buffer_size: usize,
            pub watermark_low: f32,
            pub watermark_high: f32,
            pub pressure_check_interval: Duration,
        }

        // Capture Manager Configuration
        #[derive(Debug, Clone)]
        pub struct CaptureConfig {
            pub pipeline_stages: Vec<StageConfig>,
            pub batch_size: usize,
            pub max_parallel_streams: usize,
            pub stream_timeout: Duration,
        }

        #[derive(Debug, Clone)]
        pub struct StageConfig {
            pub stage_type: StageType,
            pub thread_count: usize,
            pub queue_size: usize,
            pub batch_size: Option<usize>,
        }

        #[derive(Debug, Clone)]
        pub enum StageType {
            Ingestion,
            Parse,
            Filter,
            Process,
            Output,
        }

        // Interface Manager Configuration
        #[derive(Debug, Clone)]
        pub struct InterfaceConfig {
            pub mirror_sessions: Vec<MirrorSessionConfig>,
            pub eni_configs: Vec<ENIConfig>,
            pub hardware_offload: OffloadConfig,
            pub numa_config: Option<NumaConfig>,
            pub rx_descriptors: u16,
            pub tx_descriptors: u16,
            pub rss_queues: u16,
            pub capture_rate_limit: Option<u64>,
        }

        #[derive(Debug, Clone)]
        pub struct MirrorSessionConfig {
            pub session_id: String,
            pub source_eni: String,
            pub traffic_filters: Vec<TrafficFilter>,
            pub vni: Option<u32>,
            pub truncate_length: Option<u16>,
            pub performance_options: MirrorPerformanceOptions,
        }

        #[derive(Debug, Clone)]
        pub struct ENIConfig {
            pub eni_id: String,
            pub vpc_id: String,
            pub subnet_id: String,
            pub security_groups: Vec<String>,
            pub interface_type: ENIType,
            pub trunk_config: Option<TrunkConfig>,
        }

        #[derive(Debug, Clone)]
        pub struct OffloadConfig {
            pub dpdk_enabled: bool,
            pub xdp_enabled: bool,
            pub rx_queues: u16,
            pub tx_queues: u16,
            pub queue_size: u16,
            pub offload_features: Vec<OffloadFeature>,
        }

        // Protocol Manager Configuration
        #[derive(Debug, Clone)]
        pub struct ProtocolConfig {
            pub light_parse_depth: usize,
            pub deep_parse_enabled: bool,
            pub protocols: Vec<ProtocolSpec>,
            pub max_packet_size: usize,
        }

        #[derive(Debug, Clone)]
        pub struct ProtocolSpec {
            pub protocol: Protocol,
            pub ports: Vec<u16>,
            pub parse_options: ParseOptions,
        }

        // Filter Manager Configuration
        #[derive(Debug, Clone)]
        pub struct FilterConfig {
            pub rules: Vec<FilterRule>,
            pub default_action: FilterAction,
            pub rule_update_strategy: UpdateStrategy,
            pub hardware_offload: bool,
            pub performance_mode: FilterPerformanceMode,
        }

        #[derive(Debug, Clone)]
        pub struct FilterRule {
            pub id: String,
            pub name: String,
            pub conditions: Vec<FilterCondition>,
            pub action: FilterAction,
            pub priority: i32,
        }

        #[derive(Debug, Clone)]
        pub enum FilterCondition {
            // Layer 2
            EtherType(u16),
            MacAddress { src: Option<MacAddr>, dst: Option<MacAddr> },
            VlanId(u16),
            // Layer 3
            IpMatch(IpAddr),
            IpRange(IpNetwork),
            IpProtocol(u8),
            // Layer 4
            PortMatch(u16),
            PortRange { start: u16, end: u16 },
            TcpFlags(u8),
            // Compound
            And(Vec<FilterCondition>),
            Or(Vec<FilterCondition>),
            Not(Box<FilterCondition>),
        }

        #[derive(Debug, Clone)]
        pub enum FilterAction {
            Accept,
            Drop,
            Sample(f32),
            Mirror(String),
            Modify(PacketModification),
        }

        // Output Manager Configuration
        #[derive(Debug, Clone)]
        pub struct OutputConfig {
            pub destinations: Vec<OutputDestinationConfig>,
            pub buffer_config: OutputBufferConfig,
            pub performance: OutputPerformanceConfig,
            pub rotation: RotationConfig,
        }

        #[derive(Debug, Clone)]
        pub struct OutputDestinationConfig {
            pub id: String,
            pub destination_type: DestinationType,
            pub write_config: WriteConfig,
            pub compression: Option<CompressionConfig>,
            pub retry_policy: RetryPolicy,
        }

        #[derive(Debug, Clone)]
        pub enum DestinationType {
            S3 {
                bucket: String,
                prefix: String,
                region: String,
                storage_class: S3StorageClass,
            },
            LocalFile {
                path: PathBuf,
                sync_mode: SyncMode,
            },
            NetworkStream {
                endpoint: String,
                protocol: StreamProtocol,
                tls_config: Option<TlsConfig>,
            },
            Kafka {
                brokers: Vec<String>,
                topic: String,
                partition_strategy: PartitionStrategy,
            },
        }

        pub struct StateConfig {
            pub persistence: StatePersistenceConfig,
            pub transition_policies: Vec<TransitionPolicy>,
            pub recovery_policies: Vec<RecoveryPolicy>,
            pub pressure_policies: Vec<PressurePolicy>,
            pub health_check_interval: Duration,
        }
        
        pub struct StatePersistenceConfig {
            pub enabled: bool,
            pub storage_path: PathBuf,
            pub sync_interval: Duration,
            pub retention: Duration,
        }
        
        pub struct TransitionPolicy {
            pub from_state: CaptureState,
            pub to_state: CaptureState,
            pub conditions: Vec<TransitionCondition>,
            pub required_approvals: Vec<String>,
            pub timeout: Duration,
        }
        
        pub struct RecoveryPolicy {
            pub component: String,
            pub conditions: Vec<RecoveryCondition>,
            pub actions: Vec<RecoveryAction>,
            pub max_attempts: u32,
            pub backoff_policy: BackoffPolicy,
        }
        
        pub struct PressurePolicy {
            pub level: PressureLevel,
            pub actions: Vec<PressureAction>,
            pub thresholds: PressureThresholds,
            pub cooldown_period: Duration,
        }
        
        pub struct BackoffPolicy {
            pub initial_delay: Duration,
            pub max_delay: Duration,
            pub multiplier: f32,
            pub jitter: f32,
        }
        
        pub enum TransitionCondition {
            HealthStatus(HealthStatus),
            PressureLevel(PressureLevel),
            ComponentState(String, ComponentStatus),
            Custom(String),
        }
        
        pub enum RecoveryCondition {
            ErrorCount(u32),
            FailureRate(f32),
            TimeWindow(Duration),
            ResourceExhaustion(String),
        }

        // Storage Manager Configuration
        pub struct StorageConfig {
            pub mount_point: PathBuf,
            pub write_buffer_size: usize,
            pub max_file_size: u64,
            pub min_free_space: u64,
            pub batch_size: usize,
            pub pressure_thresholds: PressureThresholds,
        }
        
        pub struct PressureThresholds {
            pub warning_threshold: f32,
            pub critical_threshold: f32,
        }

        #[derive(Debug, Clone)]
        pub struct AuthConfig {
            pub method: AuthMethod,
            pub credentials_path: PathBuf,
            pub token_validity: Duration,
        }

        #[derive(Debug, Clone)]
        pub struct EncryptionConfig {
            pub algorithm: String,
            pub key_size: usize,
            pub key_rotation: Duration,
            pub cipher_suite: Vec<String>,
        }

        // Cloud Manager Configuration
        #[derive(Debug, Clone)]
        pub struct CloudConfig {
            pub instance_metadata: InstanceMetadata,
            pub mirror_sessions: Vec<MirrorSessionConfig>,
            pub network_interfaces: Vec<NetworkInterfaceConfig>,
            pub resource_limits: InstanceLimits,
        }

        // Telemetry Manager Configuration
        #[derive(Debug, Clone)]
        pub struct TelemetryConfig {
            pub metrics_interval: Duration,
            pub log_level: LogLevel,
            pub exporters: Vec<MetricsExporter>,
            pub pressure_alert_thresholds: PressureThresholds,
            pub sampling_rates: MetricsSamplingConfig,
        }
    }

    // Validation Framework
    pub mod validation {
        use super::*;

        #[derive(Debug)]
        pub struct ValidationResult {
            pub is_valid: bool,
            pub errors: Vec<ValidationError>,
            pub warnings: Vec<ValidationWarning>,
        }

        #[derive(Debug)]
        pub enum ValidationError {
            InvalidValue {
                path: String,
                reason: String,
            },
            MissingRequired {
                path: String,
            },
            Conflict {
                path1: String,
                path2: String,
                reason: String,
            },
            ResourceConstraint {
                path: String,
                constraint: String,
            },
        }

        #[derive(Debug)]
        pub enum ValidationWarning {
            Deprecated {
                path: String,
                alternative: String,
            },
            Performance {
                path: String,
                impact: String,
            },
            SecurityRisk {
                path: String,
                description: String,
            },
        }
    }

    // Config Manager Interface
    pub trait ConfigManager: Manager {
        fn load(&mut self) -> Result<(), Error>;
        fn get_buffer_config(&self) -> Result<configs::BufferConfig, Error>;
        fn get_capture_config(&self) -> Result<configs::CaptureConfig, Error>;
        fn get_interface_config(&self) -> Result<configs::InterfaceConfig, Error>;
        fn get_protocol_config(&self) -> Result<configs::ProtocolConfig, Error>;
        fn get_filter_config(&self) -> Result<configs::FilterConfig, Error>;
        fn get_output_config(&self) -> Result<configs::OutputConfig, Error>;
        fn get_storage_config(&self) -> Result<configs::StorageConfig, Error>;
        fn get_telemetry_config(&self) -> Result<configs::TelemetryConfig, Error>;
        fn get_security_config(&self) -> Result<configs::SecurityConfig, Error>;
        fn get_cloud_config(&self) -> Result<configs::CloudConfig, Error>;
        
        fn validate_config<T: Validate>(&self, config: &T) -> validation::ValidationResult;
        
        fn update_config<T>(&mut self, config: T) -> Result<(), Error>
        where T: Into<ConfigUpdate> + Validate;
    }

    #[derive(Debug)]
    pub enum ConfigUpdate {
        Buffer(configs::BufferConfig),
        Capture(configs::CaptureConfig),
        Interface(configs::InterfaceConfig),
        Protocol(configs::ProtocolConfig),
        Filter(configs::FilterConfig),
        Output(configs::OutputConfig),
        Storage(configs::StorageConfig),
        Telemetry(configs::TelemetryConfig),
        Security(configs::SecurityConfig),
        Cloud(configs::CloudConfig),
    }
}

// Error handling utilities and definitions
mod error {
    /// Defines common error types used across managers
    #[derive(Debug)]
    pub enum Error {
        InitializationError(String),
        RuntimeError(String),
        CommunicationError(String),
        IOError(std::io::Error),
        PressureError(PressureErrorKind),
        ResourceExhausted(ResourceKind),
        ValidationError(ValidationErrorKind),
        PerformanceError(PerformanceIssue),
    }
}