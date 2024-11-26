use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use super::test_constants::*;
use super::test_packet::{TestPacket, TestPacketError};

#[derive(Debug)]
pub enum MockInterfaceError {
    AlreadyUp,
    AlreadyDown,
    NotUp,
    QueueFull(usize),
    LockError(String),
    PacketError(TestPacketError),
}

#[derive(Debug)]
pub enum InterfaceState {
    Up,
    Down,
    Error(String),
}

pub struct MockInterface {
    name: String,
    state: Arc<Mutex<InterfaceState>>,
    packet_queue: Arc<Mutex<VecDeque<TestPacket>>>,
    max_queue_size: usize,
}

impl Default for MockInterface {
    fn default() -> Self {
        Self::new("mock0")
    }
}

impl MockInterface {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Arc::new(Mutex::new(InterfaceState::Down)),
            packet_queue: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_QUEUE_SIZE))),
            max_queue_size: MAX_QUEUE_SIZE,
        }
    }

    #[must_use]
    pub fn with_queue_size(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            state: Arc::new(Mutex::new(InterfaceState::Down)),
            packet_queue: Arc::new(Mutex::new(VecDeque::with_capacity(size))),
            max_queue_size: size,
        }
    }

    pub fn open(&mut self) -> Result<(), MockInterfaceError> {
        let mut state = self.state.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
        
        match *state {
            InterfaceState::Up => Err(MockInterfaceError::AlreadyUp),
            _ => {
                *state = InterfaceState::Up;
                Ok(())
            }
        }
    }

    pub fn close(&mut self) -> Result<(), MockInterfaceError> {
        let mut state = self.state.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
        
        match *state {
            InterfaceState::Down => Err(MockInterfaceError::AlreadyDown),
            _ => {
                *state = InterfaceState::Down;
                Ok(())
            }
        }
    }

    pub fn inject_test_packet(&self, packet: TestPacket) -> Result<(), MockInterfaceError> {
        let mut queue = self.packet_queue.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
        
        if queue.len() >= self.max_queue_size {
            return Err(MockInterfaceError::QueueFull(self.max_queue_size));
        }

        queue.push_back(packet);
        Ok(())
    }

    pub fn receive_packet(&self) -> Result<Option<TestPacket>, MockInterfaceError> {
        let state = self.state.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;

        if !matches!(*state, InterfaceState::Up) {
            return Err(MockInterfaceError::NotUp);
        }

        let mut queue = self.packet_queue.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
            
        Ok(queue.pop_front())
    }

    #[must_use]
    pub fn is_up(&self) -> bool {
        matches!(
            *self.state.lock().expect("Lock poisoned"),
            InterfaceState::Up
        )
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn queue_size(&self) -> usize {
        self.packet_queue.lock()
            .expect("Lock poisoned")
            .len()
    }

    pub fn force_error_state(&mut self, error: &str) -> Result<(), MockInterfaceError> {
        let mut state = self.state.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
        
        *state = InterfaceState::Error(error.to_string());
        Ok(())
    }

    pub fn clear_queue(&self) -> Result<(), MockInterfaceError> {
        let mut queue = self.packet_queue.lock()
            .map_err(|e| MockInterfaceError::LockError(e.to_string()))?;
        
        queue.clear();
        Ok(())
    }
}

// Add these constants to test_constants.rs
const MAX_QUEUE_SIZE: usize = 1024;
const DEFAULT_INTERFACE_NAME: &str = "mock0";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_features::*;

    fn setup_interface() -> MockInterface {
        MockInterface::new(DEFAULT_INTERFACE_NAME)
    }

    #[test]
    fn test_interface_creation() {
        let interface = setup_interface();
        assert!(!interface.is_up());
        assert_eq!(interface.name(), DEFAULT_INTERFACE_NAME);
    }

    #[test]
    fn test_interface_open_close() {
        let mut interface = setup_interface();
        assert!(interface.open().is_ok());
        assert!(interface.is_up());
        assert!(interface.close().is_ok());
        assert!(!interface.is_up());
    }

    #[test]
    fn test_double_open() {
        let mut interface = setup_interface();
        assert!(interface.open().is_ok());
        assert!(matches!(interface.open(), Err(MockInterfaceError::AlreadyUp)));
    }

    #[test]
    fn test_packet_injection() {
        let interface = setup_interface();
        let test_packet = create_test_packet(100);
        
        assert!(interface.inject_test_packet(test_packet.clone()).is_ok());
        assert_eq!(interface.queue_size(), 1);
    }

    #[test]
    fn test_packet_reception() {
        let mut interface = setup_interface();
        interface.open().unwrap();
        
        let test_packet = create_test_packet(100);
        interface.inject_test_packet(test_packet.clone()).unwrap();
        
        let received = interface.receive_packet().unwrap();
        assert!(received.is_some());
        assert_eq!(received.unwrap().length, 100);
    }

    #[test]
    fn test_queue_overflow() {
        let interface = MockInterface::with_queue_size(DEFAULT_INTERFACE_NAME, 2);
        let test_packet = create_test_packet(100);

        assert!(interface.inject_test_packet(test_packet.clone()).is_ok());
        assert!(interface.inject_test_packet(test_packet.clone()).is_ok());
        assert!(matches!(
            interface.inject_test_packet(test_packet.clone()),
            Err(MockInterfaceError::QueueFull(_))
        ));
    }

    #[test]
    fn test_receive_when_down() {
        let interface = setup_interface();
        assert!(matches!(
            interface.receive_packet(),
            Err(MockInterfaceError::NotUp)
        ));
    }

    #[test]
    fn test_error_state() {
        let mut interface = setup_interface();
        interface.open().unwrap();
        
        assert!(interface.force_error_state("Test error").is_ok());
        assert!(matches!(
            interface.receive_packet(),
            Err(MockInterfaceError::NotUp)
        ));
    }

    #[test]
    fn test_clear_queue() {
        let interface = setup_interface();
        let test_packet = create_test_packet(100);
        
        interface.inject_test_packet(test_packet.clone()).unwrap();
        assert_eq!(interface.queue_size(), 1);
        
        interface.clear_queue().unwrap();
        assert_eq!(interface.queue_size(), 0);
    }

    #[test]
    fn test_mirror_packet_handling() {
        let interface = setup_interface();
        let mirror_packet = create_mirror_packet();
        
        assert!(interface.inject_test_packet(mirror_packet).is_ok());
        assert_eq!(interface.queue_size(), 1);
    }
}