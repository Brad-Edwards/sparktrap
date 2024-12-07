// protocol/traits.rs
/// The `ProtocolManager` is responsible for parsing packets at various network layers and performing deep inspection.
///
/// Can be extended to handle custom protocols, perform DPI, or extract metadata for further analysis.
use async_trait::async_trait;
use std::collections::HashMap;

use crate::traits::{Error, HealthCheck, Lifecycle, Packet, PacketProcessor};

/// Trait for protocol analysis.
#[async_trait]
pub trait ProtocolManager: Lifecycle + PacketProcessor + HealthCheck + Send + Sync {
    /// Parses headers in the packet.
    async fn parse_headers(&mut self, packet: &mut Packet) -> Result<HeaderInfo, Error>;
    async fn parse_headers_batch(
        &mut self,
        packets: *mut Packet,
        count: usize,
    ) -> Result<(), Error>;
    /// Performs deep inspection on the packet.
    async fn deep_inspect(&mut self, packet: &mut Packet) -> Result<InspectionResult, Error>;
    async fn deep_inspect_batch(&mut self, packets: *mut Packet, count: usize)
        -> Result<(), Error>;
}

/// Information extracted from packet headers.
#[derive(Debug, Clone)]
pub struct HeaderInfo {
    pub protocols: Vec<String>,
    pub fields: HashMap<String, String>,
}

/// Result of a deep packet inspection.
#[derive(Debug, Clone)]
pub struct InspectionResult {
    pub findings: Vec<String>,
    pub anomalies: Vec<String>,
}
