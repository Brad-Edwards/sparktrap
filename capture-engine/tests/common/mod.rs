// tests/common/mod.rs

mod test_constants;
mod test_features;
mod test_packet;
mod mock_interface;
mod test_context;

pub use test_constants::*;
pub use test_features::*;
pub use test_packet::{
    TestPacket,
    TestPacketError,
    VxlanHeader,
    MirrorMetadata,
    MirrorMetadataBuilder,
};
pub use mock_interface::{
    MockInterface,
    MockInterfaceError,
    InterfaceState,
};
pub use test_context::{
    CaptureTestContext,
    TestContextError,
    CaptureConfig,
    MockBuffer,
    TestContextStats,
};
