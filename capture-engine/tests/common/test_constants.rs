// Packet size constants
pub const MAX_PACKET_SIZE: usize = 65535;  // Standard MTU limit
pub const MIN_PACKET_SIZE: usize = 14;     // Minimum ethernet frame
pub const VXLAN_HEADER_SIZE: usize = 50;   // VXLAN header size
pub const MIN_VXLAN_SIZE: usize = VXLAN_HEADER_SIZE + MIN_PACKET_SIZE;

// VXLAN constants
pub const VNI_MAX: u32 = 0xFF_FFFF;       // 24-bit max
pub const VXLAN_UDP_PORT: u16 = 4789;     // Standard VXLAN port
pub const VXLAN_FLAGS_MIRROR: u8 = 0x08;  // VPC Mirror flag

// Test data constants
pub const TEST_ENI: &str = "eni-12345678";
pub const TEST_VPC: &str = "vpc-12345678";
pub const TEST_SUBNET: &str = "subnet-12345678";
pub const TEST_ACCOUNT: &str = "123456789012";
pub const TEST_SESSION: &str = "tms-12345678";
pub const TEST_FILTER: &str = "tmf-12345678";