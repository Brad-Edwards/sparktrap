// interface/traits.rs
// `InterfaceManager` deals with network interfaces where packets are captured.
use crate::traits::{Error, EventHandler, Lifecycle, Packet, PressureAware};
///
/// This abstraction allows plugging in different backend implementations:
/// - AWS: ENA driver capture or VPC Traffic Mirror sessions.
/// - On-prem: DPDK or AF_PACKET-based capture.
///
/// The engine just needs the `InterfaceManager` trait to work, no matter where it runs.
use async_trait::async_trait;

/// Events specific to interface management.
#[derive(Debug)]
pub enum InterfaceEvent<'a> {
    InterfaceUp(String),
    InterfaceDown(String),
    PacketReceived(Packet<'a>),
    PacketDrop(PacketDropInfo),
    LinkStatusChange(LinkStatus),
}

/// Information about a packet drop.
#[derive(Debug)]
pub struct PacketDropInfo {
    pub interface_id: String,
    pub reason: String,
}

/// Status of a network link.
#[derive(Debug, Clone)]
pub enum LinkStatus {
    Up,
    Down,
    Unknown,
}

/// Trait for managing network interfaces.
#[async_trait]
pub trait InterfaceManager<'a>:
    Lifecycle + EventHandler<InterfaceEvent<'a>> + PressureAware + Send + Sync
{
    /// Captures packets from the interface.
    async fn capture_packets(&mut self) -> Result<Vec<Packet>, Error>;

    /// Configures the network interface.
    async fn configure_interface(&mut self, config: InterfaceConfig) -> Result<(), Error>;

    /// Retrieves the status of the interface.
    fn interface_status(&self) -> InterfaceStatus;

    /// Sets the capture rate limit.
    fn set_capture_rate_limit(&mut self, limit: Option<u64>) -> Result<(), Error>;
}

/// Configuration for a network interface.
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub interface_id: String,
    pub promiscuous_mode: bool,
    pub offload_enabled: bool,
}

/// Status of the network interface.
#[derive(Debug, Clone)]
pub struct InterfaceStatus {
    pub interface_id: String,
    pub link_status: LinkStatus,
    pub speed_mbps: Option<u64>,
    pub duplex: Option<String>,
    pub errors: Vec<String>,
}
