use std::time::SystemTime;
use crate::common::test_constants::{
    MAX_PACKET_SIZE,
    MIN_PACKET_SIZE,
    VXLAN_HEADER_SIZE,
    MIN_VXLAN_SIZE,
    VNI_MAX,
    VXLAN_UDP_PORT,
    VXLAN_FLAGS_MIRROR,
};

/// Represents errors that can occur during packet operations
#[derive(Debug)]
pub enum TestPacketError {
    PacketTooLarge(usize),
    PacketTooSmall(usize),
    InvalidVNI(u32),
    InvalidPort(u16),
    InvalidLength(usize),
    ValidationError(String),
    MetadataError(String),
}

/// VXLAN header structure for mirror packets
#[derive(Debug, Clone)]
pub struct VxlanHeader {
    pub vni: u32,
    pub udp_dest_port: u16,
    pub vxlan_flags: u8,
}

impl VxlanHeader {
    pub fn new(vni: u32) -> Result<Self, TestPacketError> {
        if vni > VNI_MAX {
            return Err(TestPacketError::InvalidVNI(vni));
        }

        Ok(Self {
            vni,
            udp_dest_port: VXLAN_UDP_PORT,
            vxlan_flags: VXLAN_FLAGS_MIRROR,
        })
    }

    pub fn validate(&self) -> Result<(), TestPacketError> {
        if self.vni > VNI_MAX {
            return Err(TestPacketError::InvalidVNI(self.vni));
        }
        if self.udp_dest_port != VXLAN_UDP_PORT {
            return Err(TestPacketError::InvalidPort(self.udp_dest_port));
        }
        if self.vxlan_flags & VXLAN_FLAGS_MIRROR != VXLAN_FLAGS_MIRROR {
            return Err(TestPacketError::ValidationError(
                "Invalid VXLAN flags for VPC Mirror".to_string()
            ));
        }
        Ok(())
    }
}

/// Metadata for mirrored packets
#[derive(Debug, Clone)]
pub struct MirrorMetadata {
    source_eni: String,
    destination_eni: String,
    filter_id: String,
    session_id: String,
    account_id: String,
    vpc_id: String,
    subnet_id: String,
}

/// Builder for MirrorMetadata
#[derive(Default)]
pub struct MirrorMetadataBuilder {
    source_eni: Option<String>,
    destination_eni: Option<String>,
    filter_id: Option<String>,
    session_id: Option<String>,
    account_id: Option<String>,
    vpc_id: Option<String>,
    subnet_id: Option<String>,
}

impl MirrorMetadataBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn source_eni(mut self, eni: &str) -> Self {
        self.source_eni = Some(eni.to_string());
        self
    }

    pub fn destination_eni(mut self, eni: &str) -> Self {
        self.destination_eni = Some(eni.to_string());
        self
    }

    pub fn filter_id(mut self, id: &str) -> Self {
        self.filter_id = Some(id.to_string());
        self
    }

    pub fn session_id(mut self, id: &str) -> Self {
        self.session_id = Some(id.to_string());
        self
    }

    pub fn account_id(mut self, id: &str) -> Self {
        self.account_id = Some(id.to_string());
        self
    }

    pub fn vpc_id(mut self, id: &str) -> Self {
        self.vpc_id = Some(id.to_string());
        self
    }

    pub fn subnet_id(mut self, id: &str) -> Self {
        self.subnet_id = Some(id.to_string());
        self
    }

    pub fn build(self) -> Result<MirrorMetadata, TestPacketError> {
        Ok(MirrorMetadata {
            source_eni: self.source_eni.ok_or_else(|| TestPacketError::MetadataError("Missing source ENI".to_string()))?,
            destination_eni: self.destination_eni.ok_or_else(|| TestPacketError::MetadataError("Missing destination ENI".to_string()))?,
            filter_id: self.filter_id.ok_or_else(|| TestPacketError::MetadataError("Missing filter ID".to_string()))?,
            session_id: self.session_id.ok_or_else(|| TestPacketError::MetadataError("Missing session ID".to_string()))?,
            account_id: self.account_id.ok_or_else(|| TestPacketError::MetadataError("Missing account ID".to_string()))?,
            vpc_id: self.vpc_id.ok_or_else(|| TestPacketError::MetadataError("Missing VPC ID".to_string()))?,
            subnet_id: self.subnet_id.ok_or_else(|| TestPacketError::MetadataError("Missing subnet ID".to_string()))?,
        })
    }
}

/// Represents a test packet with optional VXLAN encapsulation
#[derive(Debug, Clone)]
pub struct TestPacket {
    pub data: Vec<u8>,
    pub timestamp: SystemTime,
    pub length: usize,
    pub is_vxlan: bool,
    pub vxlan_header: Option<VxlanHeader>,
    pub inner_packet: Option<Box<TestPacket>>,
    pub truncated: bool,
    pub fragmented: bool,
    pub mirror_metadata: Option<MirrorMetadata>,
}

impl TestPacket {
    pub fn new(data: Vec<u8>) -> Result<Self, TestPacketError> {
        if data.len() > MAX_PACKET_SIZE {
            return Err(TestPacketError::PacketTooLarge(data.len()));
        }
        if data.len() < MIN_PACKET_SIZE {
            return Err(TestPacketError::PacketTooSmall(data.len()));
        }

        Ok(Self {
            length: data.len(),
            data,
            timestamp: SystemTime::now(),
            is_vxlan: false,
            vxlan_header: None,
            inner_packet: None,
            truncated: false,
            fragmented: false,
            mirror_metadata: None,
        })
    }

    pub fn new_mirror_packet(inner: TestPacket, vni: u32) -> Result<Self, TestPacketError> {
        let vxlan_header = VxlanHeader::new(vni)?;
        let total_length = inner.length + VXLAN_HEADER_SIZE;
        
        if total_length > MAX_PACKET_SIZE {
            return Err(TestPacketError::PacketTooLarge(total_length));
        }

        Ok(Self {
            length: total_length,
            data: vec![0; total_length],
            timestamp: SystemTime::now(),
            is_vxlan: true,
            vxlan_header: Some(vxlan_header),
            inner_packet: Some(Box::new(inner)),
            truncated: false,
            fragmented: false,
            mirror_metadata: None,
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn is_mirror_packet(&self) -> bool {
        self.is_vxlan && self.vxlan_header.is_some()
    }

    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    pub fn truncate(&mut self, new_length: usize) {
        if new_length < self.length {
            self.length = new_length;
            self.data.truncate(new_length);
            self.truncated = true;
        }
    }

    pub fn set_mirror_metadata(&mut self, metadata: MirrorMetadata) {
        self.mirror_metadata = Some(metadata);
    }

    pub fn validate(&self) -> Result<(), TestPacketError> {
        if self.length > MAX_PACKET_SIZE {
            return Err(TestPacketError::PacketTooLarge(self.length));
        }
        if self.length < MIN_PACKET_SIZE {
            return Err(TestPacketError::PacketTooSmall(self.length));
        }
        if self.is_vxlan {
            if let Some(header) = &self.vxlan_header {
                header.validate()?;
            } else {
                return Err(TestPacketError::ValidationError("Missing VXLAN header".to_string()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_creation() {
        let packet = TestPacket::new(vec![0; 100]).unwrap();
        assert_eq!(packet.len(), 100);
        assert!(!packet.is_mirror_packet());
    }

    #[test]
    fn test_packet_too_large() {
        let result = TestPacket::new(vec![0; MAX_PACKET_SIZE + 1]);
        assert!(matches!(result, Err(TestPacketError::PacketTooLarge(_))));
    }

    #[test]
    fn test_packet_too_small() {
        let result = TestPacket::new(vec![0; MIN_PACKET_SIZE - 1]);
        assert!(matches!(result, Err(TestPacketError::PacketTooSmall(_))));
    }

    #[test]
    fn test_mirror_packet() {
        let inner = TestPacket::new(vec![0; 100]).unwrap();
        let mirror = TestPacket::new_mirror_packet(inner, 12345).unwrap();
        assert!(mirror.is_mirror_packet());
        assert_eq!(mirror.len(), 100 + VXLAN_HEADER_SIZE);
    }

    #[test]
    fn test_invalid_vni() {
        let inner = TestPacket::new(vec![0; 100]).unwrap();
        let result = TestPacket::new_mirror_packet(inner, VNI_MAX + 1);
        assert!(matches!(result, Err(TestPacketError::InvalidVNI(_))));
    }

    #[test]
    fn test_truncation() {
        let mut packet = TestPacket::new(vec![0; 100]).unwrap();
        packet.truncate(50);
        assert!(packet.is_truncated());
        assert_eq!(packet.len(), 50);
    }

    #[test]
    fn test_metadata() {
        let mut packet = TestPacket::new(vec![0; 100]).unwrap();
        let metadata = MirrorMetadataBuilder::new()
            .source_eni("eni-1")
            .destination_eni("eni-2")
            .filter_id("filter-1")
            .session_id("session-1")
            .account_id("account-1")
            .vpc_id("vpc-1")
            .subnet_id("subnet-1")
            .build()
            .unwrap();
        
        packet.set_mirror_metadata(metadata);
        assert!(packet.mirror_metadata.is_some());
    }
}