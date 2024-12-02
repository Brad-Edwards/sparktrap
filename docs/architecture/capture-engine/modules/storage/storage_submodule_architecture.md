# Storage Submodule Architecture

## Core Submodules and Responsibilities

### storage_manager

Core storage orchestration and policy enforcement.

- Coordinates all storage operations across submodules
- Implements storage policies and quotas
- Routes operations to appropriate storage types
- Reports status to `telemetry`
- Maintains state via `state`
- Handles backpressure signaling
- Coordinates with `cloud` for AWS integration

### nvme_manager

High-performance NVMe storage operations for packet capture.

- Manages NVMe device allocation and access
- Implements kernel bypass for direct access
- Handles wear leveling and drive health
- Provides buffers to `buffer` module
- Reports performance metrics to `telemetry`
- Implements pressure monitoring
- Maintains device state maps

### state_index

Combined state persistence and indexing operations.

- Manages configuration and state persistence
- Maintains metadata indexes for fast lookup
- Handles state snapshots and recovery
- Implements search operations
- Manages index compression
- Reports index health to `telemetry`
- Coordinates with `state` module

### lifecycle

Data retention and cleanup management.

- Implements retention policies
- Executes secure data deletion
- Manages storage cleanup operations
- Tracks data lifecycle states
- Maintains audit logs
- Reports lifecycle events to `telemetry`
- Coordinates with `security` for secure deletion

### io_pipeline

Storage I/O path management and optimization.

- Handles concurrent storage access
- Manages compression operations
- Implements write buffering
- Provides isolation between operations
- Handles I/O error conditions
- Reports I/O metrics to `telemetry`
- Maintains I/O queues and priorities

### buffer_interface

Optimized buffer management integration.

- Coordinates with `buffer` module
- Handles memory pressure signals
- Manages buffer allocation requests
- Implements zero-copy paths where possible
- Reports buffer utilization
- Maintains buffer maps
- Handles buffer recycling

## Key Interactions

### Performance Path

- `buffer_interface` → `nvme_manager` for high-speed capture
- Direct memory mapping for zero-copy operations
- Kernel bypass for NVMe access
- Pressure signaling for flow control

### Control Path

- `storage_manager` orchestration of all submodules
- Policy enforcement and quota management
- AWS service integration coordination
- State synchronization and recovery

### Data Path

- Packet capture via `buffer_interface`
- Compression through `io_pipeline`
- Index updates via `state_index`
- Lifecycle tracking and cleanup

## Operational Requirements

### Performance

- Maintains 30 Gbps sustained throughput
- Sub-millisecond buffer operations
- Efficient burst handling
- Memory pressure management

### Reliability

- Handles component failures
- Supports degraded operations
- Maintains data integrity
- Enables recovery operations

### Security

- Encryption at rest
- Secure deletion
- Access control
- Audit logging

### Monitoring

- Performance metrics
- Health status
- Resource utilization
- Pressure indicators

## Integration Points

### Internal

- `buffer` module for memory management
- `state` module for state coordination
- `telemetry` for monitoring
- `security` for encryption

### External

- AWS KMS for encryption
- CloudWatch for metrics
- AWS Backup for snapshots
- Instance metadata service
