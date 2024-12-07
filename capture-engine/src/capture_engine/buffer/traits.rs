// buffer/traits.rs
/// The `BufferManager` controls buffer allocation and release.
use async_trait::async_trait;

use crate::traits::{BufferId, Error, Lifecycle, PressureAware, PressureLevel, PressureStatus};
/// Represents events specific to buffer management.
#[derive(Debug)]
pub enum BufferEvent {
    MemoryPressure(PressureLevel),
    BufferReleased(BufferId),
    PoolExhausted,
    WatermarkCrossed(WatermarkType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatermarkType {
    Low,
    High,
    Critical,
}

/// Trait for managing buffers.
#[async_trait]
pub trait BufferManager: Lifecycle + PressureAware + Send + Sync {
    async fn acquire_buffer(&mut self, size: usize) -> Result<BufferHandle, Error>;
    async fn release_buffer(&mut self, buffer_id: BufferId) -> Result<(), Error>;
    fn memory_pressure_status(&self) -> PressureStatus;
}

// A handle that can provide &mut [u8] directly, avoiding arc+trait overhead
pub struct BufferHandle {
    pub buffer_id: BufferId,
    pub data: *mut u8,
    pub capacity: usize,
}

/// Represents a buffer managed by `BufferManager`.
pub trait ManagedBuffer: Send + Sync {
    // Provides a read-only view of the buffer's data.
    fn as_slice(&self) -> &[u8];

    // Provides a mutable view of the buffer's data.
    fn as_mut_slice(&mut self) -> &mut [u8];

    // Retrieves metadata associated with the buffer.
    fn metadata(&self) -> &BufferMetadata;
}

/// Zero-copy buffer operations for high-performance packet handling
/// Zero-copy buffer trait for efficient packet data handling
pub trait ZeroCopyBuffer {
    /// Returns raw const pointer to buffer data
    ///
    /// # Safety
    /// Caller must ensure:
    /// - Pointer remains valid for buffer lifetime
    /// - No mutable references exist while using pointer
    /// - Accessed memory remains within buffer bounds
    unsafe fn as_ptr(&self) -> *const u8;

    /// Returns raw mutable pointer to buffer data
    ///
    /// # Safety
    /// Caller must ensure:
    /// - Pointer remains valid for buffer lifetime
    /// - No other references exist while using pointer
    /// - Accessed memory remains within buffer bounds
    unsafe fn as_mut_ptr(&mut self) -> *mut u8;

    /// Returns length of buffer in bytes
    fn len(&self) -> usize;

    /// Returns true if buffer contains no data
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ZeroCopyBuffer for BufferHandle {
    unsafe fn as_ptr(&self) -> *const u8 {
        self.data
    }
    unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data
    }
    fn len(&self) -> usize {
        self.capacity
    }
}

/// NUMA-aware buffer management for optimized memory access
pub trait NumaAwareBufferManager: BufferManager {
    fn set_numa_node(&mut self, node_id: u32) -> Result<(), Error>;
    fn allocate_on_node(&mut self, size: usize, node_id: u32) -> Result<BufferHandle, Error>;
}

/// Metadata associated with a buffer.
pub struct BufferMetadata {
    pub buffer_id: BufferId,
    pub capacity: usize,
}
