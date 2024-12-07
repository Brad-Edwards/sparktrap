// orchestrator/traits.rs
use crate::capture_engine::capture::traits::CaptureManager;
use crate::capture_engine::cloud::traits::CloudEvent;
use crate::capture_engine::cloud::traits::CloudManager;
use crate::capture_engine::control::traits::ControlEvent;
use crate::capture_engine::control::traits::ControlManager;
use crate::capture_engine::event::traits::SystemEvent;
use crate::capture_engine::interface::traits::InterfaceEvent;
use crate::capture_engine::interface::traits::InterfaceManager;
use crate::capture_engine::output::traits::OutputEvent;
use crate::capture_engine::output::traits::OutputManager;
use crate::capture_engine::protocol::traits::ProtocolManager;
use crate::capture_engine::security::traits::SecurityEvent;
use crate::capture_engine::security::traits::SecurityManager;
use crate::capture_engine::state::traits::StateEvent;
use crate::capture_engine::state::traits::StateManager;
use crate::capture_engine::storage::traits::StorageEvent;
use crate::capture_engine::storage::traits::StorageManager;
use crate::capture_engine::telemetry::traits::TelemetryManager;
/// Top-level orchestrator and pipeline definitions.
///
/// The orchestrator is intended to be driven by an external control layer
/// that handles lifecycle and scaling.

/// `PacketPath` combines a `PacketProcessor` trait with other responsibilities to define
/// how packets flow through the system.
use crate::traits::{EventHandler, PacketProcessor};
use std::sync::mpsc;

pub trait PacketPath: PacketProcessor {}

/// `ExternalEventDriven` indicates that an implementing component responds to system-level events.
pub trait ExternalEventDriven: EventHandler<SystemEvent> {}

/// `CorePipeline` defines the minimal pipeline segments: capture, protocol, and output management.
///
/// The generic parameter `M` must implement `CaptureManager`, `ProtocolManager`, and `OutputManager`.
pub struct CorePipeline<C: CaptureManager, P: ProtocolManager, O: OutputManager> {
    /// `capture` represents the combined pipeline segment that handles packet ingestion,
    /// parsing, and eventual output writing.
    pub capture: C,
    pub protocol: P,
    pub output: O,
}

/// `EventDrivenManagers` holds managers that respond to events.
pub struct EventDrivenManagers<
    C: ControlManager + EventHandler<ControlEvent> + Send + Sync,
    Cl: CloudManager + EventHandler<CloudEvent> + Send + Sync,
    S: SecurityManager + EventHandler<SecurityEvent> + Send + Sync,
    St: StateManager + EventHandler<StateEvent> + Send + Sync,
    I: for<'a> InterfaceManager<'a> + for<'a> EventHandler<InterfaceEvent<'a>> + Send + Sync,
    O: OutputManager + EventHandler<OutputEvent> + Send + Sync,
    T: TelemetryManager + Send + Sync,
    Sm: StorageManager + EventHandler<StorageEvent> + Send + Sync,
> {
    pub control: C,
    pub cloud: Cl,
    pub security: S,
    pub state: St,
    pub interface: I,
    pub output: O,
    pub telemetry: T,
    pub storage: Sm,
}

/// `HotPathModules` must operate at line rate or with minimal latency.
///
/// HRTB since packets have different lifetimes, managers must
/// be able to handle them, and don't want to store lifetime
/// information in the struct.
pub struct HotPathModules<
    I: for<'a> InterfaceManager<'a> + Send + Sync,
    C: CaptureManager + Send + Sync,
    P: ProtocolManager + Send + Sync,
    F: PacketProcessor + Send + Sync,
    O: OutputManager + Send + Sync,
> {
    pub interface: I,
    pub capture: C,
    pub protocol: P,
    pub filter: F,
    pub output: O,
}

/// The `Orchestrator` includes managers that handle events from control,
/// cloud, security, state, interface, output, storage, and telemetry.
///
/// Receivers for their respective events are included.
pub struct Orchestrator<
    'a,
    C: ControlManager + EventHandler<ControlEvent>,
    Cl: CloudManager + EventHandler<CloudEvent>,
    S: SecurityManager + EventHandler<SecurityEvent>,
    St: StateManager + EventHandler<StateEvent>,
    I: InterfaceManager<'a> + EventHandler<InterfaceEvent<'a>>,
    O: OutputManager + EventHandler<OutputEvent>,
    T: TelemetryManager,
    Sm: StorageManager + EventHandler<StorageEvent>,
> {
    pub control: C,
    pub cloud: Cl,
    pub security: S,
    pub state: St,
    pub interface: I,
    pub output: O,
    pub storage: Sm,
    pub telemetry: T,
    /// Event receivers could be connected to external sources (like AWS SQS, or pub/sub systems)
    /// but are abstracted away here.
    pub control_rx: mpsc::Receiver<ControlEvent>,
    pub cloud_rx: mpsc::Receiver<CloudEvent>,
    pub security_rx: mpsc::Receiver<SecurityEvent>,
    pub state_rx: mpsc::Receiver<StateEvent>,
    pub interface_rx: mpsc::Receiver<InterfaceEvent<'a>>,
    pub output_rx: mpsc::Receiver<OutputEvent>,
    pub storage_rx: mpsc::Receiver<StorageEvent>,
}
