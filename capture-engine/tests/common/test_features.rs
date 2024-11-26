use crate::common::{
    TestPacket,
    TestPacketError,
    MirrorMetadata,
    VxlanHeader,
    MirrorMetadataBuilder,
};

use crate::common::test_constants::{
    MAX_PACKET_SIZE,
    MIN_PACKET_SIZE,
    TEST_ENI,
    TEST_VPC,
    TEST_SUBNET,
    TEST_ACCOUNT,
    TEST_SESSION,
    TEST_FILTER,
    VNI_MAX,
};

/// Creates a basic test packet of specified size
#[must_use]
pub fn create_test_packet(size: usize) -> TestPacket {
    TestPacket::new(vec![0; size])
        .expect("Failed to create test packet")
}

/// Creates a sequence of test packets
#[must_use]
pub fn create_packet_sequence(count: usize, size: usize) -> Vec<TestPacket> {
    (0..count)
        .map(|_| create_test_packet(size))
        .collect()
}

/// Creates a test metadata structure with default values
#[must_use]
pub fn create_test_metadata() -> MirrorMetadata {
    MirrorMetadataBuilder::new()
        .source_eni(TEST_ENI)
        .destination_eni("eni-87654321")
        .filter_id(TEST_FILTER)
        .session_id(TEST_SESSION)
        .account_id(TEST_ACCOUNT)
        .vpc_id(TEST_VPC)
        .subnet_id(TEST_SUBNET)
        .build()
        .expect("Failed to build test metadata")
}

/// Creates a mirror packet with default settings
#[must_use]
pub fn create_mirror_packet() -> TestPacket {
    TestPacket::new_mirror_packet(
        create_test_packet(100),
        12345
    ).expect("Failed to create mirror packet")
}

/// Creates a sequence of mirror packets
#[must_use]
pub fn create_mirror_packet_sequence(count: usize) -> Vec<TestPacket> {
    (0..count)
        .map(|_| create_mirror_packet())
        .collect()
}

/// Creates an oversized packet for error testing
pub fn create_oversized_packet() -> Result<TestPacket, TestPacketError> {
    TestPacket::new(vec![0; MAX_PACKET_SIZE + 1])
}

/// Creates a packet with invalid VNI for error testing
pub fn create_invalid_vni_packet() -> Result<TestPacket, TestPacketError> {
    TestPacket::new_mirror_packet(
        create_test_packet(100),
        VNI_MAX + 1
    )
}

/// Creates a truncated packet for testing
#[must_use]
pub fn create_truncated_packet(size: usize) -> TestPacket {
    let mut packet = create_test_packet(size);
    packet.truncate(size / 2);
    packet
}

/// Creates a mirror packet with metadata
#[must_use]
pub fn create_mirror_packet_with_metadata() -> TestPacket {
    let mut packet = create_mirror_packet();
    packet.set_mirror_metadata(create_test_metadata());
    packet
}

/// Test scenario configuration for packet testing
#[derive(Debug)]
pub struct TestScenario {
    /// Collection of test packets
    pub packets: Vec<TestPacket>,
    /// Expected size of the packet collection
    pub expected_size: usize,
    /// Whether the scenario includes mirror packets
    pub has_mirror: bool,
    /// Whether the scenario includes metadata
    pub has_metadata: bool,
}

impl TestScenario {
    /// Creates a new basic test scenario
    #[must_use]
    pub fn new_basic(count: usize, size: usize) -> Self {
        Self {
            packets: create_packet_sequence(count, size),
            expected_size: count,
            has_mirror: false,
            has_metadata: false,
        }
    }

    /// Creates a new mirror packet test scenario
    #[must_use]
    pub fn new_mirror(count: usize) -> Self {
        Self {
            packets: create_mirror_packet_sequence(count),
            expected_size: count,
            has_mirror: true,
            has_metadata: false,
        }
    }

    /// Creates a new metadata test scenario
    #[must_use]
    pub fn new_with_metadata(count: usize) -> Self {
        let packets = (0..count)
            .map(|_| create_mirror_packet_with_metadata())
            .collect();
        
        Self {
            packets,
            expected_size: count,
            has_mirror: true,
            has_metadata: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_packet_creation() {
        let packet = create_test_packet(100);
        assert_eq!(packet.len(), 100);
        assert!(!packet.is_mirror_packet());
    }

    #[test]
    fn test_mirror_sequence() {
        let packets = create_mirror_packet_sequence(3);
        assert_eq!(packets.len(), 3);
        assert!(packets.iter().all(|p| p.is_mirror_packet()));
    }

    #[test]
    fn test_error_simulation() {
        assert!(create_invalid_vni_packet().is_err());
        assert!(create_oversized_packet().is_err());
    }

    #[test]
    fn test_scenario_creation() {
        let basic = TestScenario::new_basic(5, 100);
        assert_eq!(basic.packets.len(), 5);
        assert!(!basic.has_mirror);

        let mirror = TestScenario::new_mirror(3);
        assert_eq!(mirror.packets.len(), 3);
        assert!(mirror.has_mirror);

        let metadata = TestScenario::new_with_metadata(2);
        assert!(metadata.has_metadata);
        assert!(metadata.has_mirror);
    }

    #[test]
    fn test_truncated_packet() {
        let packet = create_truncated_packet(1500);
        assert!(packet.is_truncated());
        assert_eq!(packet.len(), 750);
    }

    #[test]
    fn test_metadata_creation() {
        let metadata = create_test_metadata();
        assert_eq!(metadata.source_eni(), TEST_ENI);
        assert_eq!(metadata.vpc_id(), TEST_VPC);
    }
}