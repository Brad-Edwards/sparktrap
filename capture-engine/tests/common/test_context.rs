use std::sync::{Arc, Mutex};

use crate::common::{
    MockInterface,
    MockInterfaceError,
    TestPacket,
    test_features::{create_test_packet, create_mirror_packet},
};

/// Default configuration values
const DEFAULT_INTERFACE_NAME: &str = "test0";
const DEFAULT_BUFFER_SIZE: usize = 8192;
const DEFAULT_SNAPLEN: usize = 65535;
const DEFAULT_TIMEOUT_MS: u32 = 1000;

/// Configuration for capture test context
#[derive(Debug, Clone)]
pub struct CaptureConfig {
    pub interface_name: String,
    pub buffer_size: usize,
    pub promiscuous: bool,
    pub snaplen: usize,
    pub timeout_ms: u32,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            interface_name: DEFAULT_INTERFACE_NAME.to_string(),
            buffer_size: DEFAULT_BUFFER_SIZE,
            promiscuous: true,
            snaplen: DEFAULT_SNAPLEN,
            timeout_ms: DEFAULT_TIMEOUT_MS,
        }
    }
}

/// Mock buffer implementation for testing
#[derive(Debug)]
pub struct MockBuffer {
    size: usize,
    used: Arc<Mutex<usize>>,
}

impl MockBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            used: Arc::new(Mutex::new(0)),
        }
    }

    pub fn available_space(&self) -> usize {
        let used = *self.used.lock().unwrap();
        self.size.saturating_sub(used)
    }
}

/// Error types for test context operations
#[derive(Debug)]
pub enum TestContextError {
    InterfaceError(String),
    BufferError(String),
    ConfigurationError(String),
    SetupError(String),
    TeardownError(String),
}

/// Statistics for test context execution
#[derive(Debug)]
pub struct TestContextStats {
    /// Number of packets currently in the queue
    pub packets_queued: usize,
    /// Available space in the buffer
    pub buffer_available: usize,
    /// Current interface status
    pub is_interface_up: bool,
}

/// Main test context for capture operations
pub struct CaptureTestContext {
    pub interface: MockInterface,
    pub config: CaptureConfig,
    buffer: MockBuffer,
}

impl CaptureTestContext {
    /// Creates a new test context with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(CaptureConfig::default())
    }

    /// Creates a new test context with custom configuration
    #[must_use]
    pub fn with_config(config: CaptureConfig) -> Self {
        Self {
            interface: MockInterface::new(&config.interface_name),
            buffer: MockBuffer::new(config.buffer_size),
            config,
        }
    }

    /// Sets up the test context
    pub fn setup(&mut self) -> Result<(), TestContextError> {
        self.interface.open()
            .map_err(|e| TestContextError::SetupError(format!("Failed to open interface: {:?}", e)))
    }

    /// Tears down the test context
    pub fn teardown(&mut self) -> Result<(), TestContextError> {
        self.interface.close()
            .map_err(|e| TestContextError::TeardownError(format!("Failed to close interface: {:?}", e)))
    }

    /// Injects a specified number of test packets
    pub fn inject_packets(&self, count: usize) -> Result<(), TestContextError> {
        for _ in 0..count {
            let packet = create_test_packet(100);
            self.interface.inject_test_packet(packet)
                .map_err(|e| TestContextError::InterfaceError(format!("Failed to inject packet: {:?}", e)))?;
        }
        Ok(())
    }

    /// Injects a specified number of mirror packets
    pub fn inject_mirror_packets(&self, count: usize) -> Result<(), TestContextError> {
        for _ in 0..count {
            let packet = create_mirror_packet();
            self.interface.inject_test_packet(packet)
                .map_err(|e| TestContextError::InterfaceError(format!("Failed to inject mirror packet: {:?}", e)))?;
        }
        Ok(())
    }

    /// Simulates an error condition
    pub fn simulate_error_condition(&mut self, error: &str) -> Result<(), TestContextError> {
        self.interface.force_error_state(error)
            .map_err(|e| TestContextError::InterfaceError(format!("Failed to simulate error: {:?}", e)))
    }

    /// Clears the interface queue
    pub fn clear_interface(&mut self) -> Result<(), TestContextError> {
        self.interface.clear_queue()
            .map_err(|e| TestContextError::InterfaceError(format!("Failed to clear interface: {:?}", e)))
    }

    /// Returns current test context statistics
    #[must_use]
    pub fn get_stats(&self) -> TestContextStats {
        TestContextStats {
            packets_queued: self.interface.queue_size(),
            buffer_available: self.buffer.available_space(),
            is_interface_up: self.interface.is_up(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = CaptureTestContext::new();
        assert!(!context.interface.is_up());
        assert_eq!(context.config.interface_name, "test0");
    }

    #[test]
    fn test_custom_config() {
        let config = CaptureConfig {
            interface_name: "test1".to_string(),
            buffer_size: 2048,
            promiscuous: false,
            snaplen: 1500,
            timeout_ms: 500,
        };
        
        let context = CaptureTestContext::with_config(config);
        assert_eq!(context.config.interface_name, "test1");
        assert_eq!(context.config.buffer_size, 2048);
    }

    #[test]
    fn test_context_setup_teardown() {
        let mut context = CaptureTestContext::new();
        assert!(context.setup().is_ok());
        assert!(context.interface.is_up());
        assert!(context.teardown().is_ok());
        assert!(!context.interface.is_up());
    }

    #[test]
    fn test_packet_injection() {
        let mut context = CaptureTestContext::new();
        context.setup().unwrap();
        
        assert!(context.inject_packets(5).is_ok());
        let stats = context.get_stats();
        assert_eq!(stats.packets_queued, 5);
    }

    #[test]
    fn test_mirror_packet_injection() {
        let mut context = CaptureTestContext::new();
        context.setup().unwrap();
        
        assert!(context.inject_mirror_packets(3).is_ok());
        let stats = context.get_stats();
        assert_eq!(stats.packets_queued, 3);
    }

    #[test]
    fn test_error_simulation() {
        let mut context = CaptureTestContext::new();
        context.setup().unwrap();
        
        assert!(context.simulate_error_condition("Test error").is_ok());
        assert!(!context.interface.is_up());
    }

    #[test]
    fn test_buffer_capacity() {
        let context = CaptureTestContext::new();
        let stats = context.get_stats();
        assert_eq!(stats.buffer_available, DEFAULT_BUFFER_SIZE);
    }
}