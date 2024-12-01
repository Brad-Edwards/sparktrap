# Capture Engine Module Architecture

## Core Modules and Responsibilities

### buffer

Manages packet memory allocation and lifecycle. Provides efficient buffer pools and handles memory pressure.

- Interfaces with `storage` for NVMe buffering
- Provides buffers to `capture` for packet storage
- Reports status to `telemetry`
- Responds to pressure signals from `state`

### capture

Core packet capture logic and orchestration.

- Uses `interface` for packet ingestion
- Requests buffers from `buffer`
- Coordinates with `protocol.headers` for initial analysis
- Applies filters via `filter`
- Routes accepted packets to `protocol.deep`
- Sends packets to `output`
- Reports to `telemetry`
- Maintains state via `state`

### cloud

Handles AWS lifecycle events and metadata.

- Notifies `state` of lifecycle events
- Coordinates with `control` for graceful shutdown
- Manages AWS credentials with `security`
- Reports cloud events to `telemetry`

### control

Manages communication with control plane.

- Receives commands and distributes to appropriate modules
- Reports status via `telemetry`
- Coordinates state changes through `state`
- Handles configuration updates

### filter

Implements packet filtering logic.

- Receives packets and header analysis from `protocol.headers`
- May interact with `interface` for hardware offload
- Reports filter stats to `telemetry`
- Maintains filter state in `state`

### interface

Manages network interfaces and packet reception.

- Handles kernel bypass (DPDK/XDP)
- Provides packets to `capture`
- Reports interface stats to `telemetry`
- Maintains interface state in `state`

### output

Manages captured data output handling.

- Receives packets from `capture`
- Uses `storage` for buffering
- Reports output stats to `telemetry`
- Maintains output state in `state`

### protocol

Handles protocol analysis and processing at two levels:

- headers: Quick L2-L4 header parsing before filtering
- deep: Full protocol analysis post-filtering

- Provides header analysis to `filter`
- Performs deep protocol analysis on filtered packets
- Reports protocol stats to `telemetry`
- Maintains protocol state in `state`

### security

Manages security and credentials.

- Provides credential management for all modules
- Interfaces with `cloud` for AWS credentials
- Reports security events to `telemetry`
- Maintains security state in `state`

### state

Manages state machines and transitions.

- Provides state management for all modules
- Uses `storage` for state persistence
- Reports state changes to `telemetry`
- Coordinates with `control` for state sync

### storage

Manages local storage resources.

- Provides NVMe management for `buffer`
- Stores telemetry data for `telemetry`
- Handles storage pressure and cleanup
- Reports storage stats to `telemetry`

### telemetry

Handles metrics, logging, and monitoring.

- Collects metrics from all modules
- Uses `storage` for local buffering
- Reports to control plane via `control`
- Maintains telemetry state in `state`

## Key Interactions

1. Packet Flow:

   ```mermaid
   graph LR
   interface --> capture --> protocol.headers --> filter --> protocol.deep --> output

2. State Management:

   ```mermaid
    graph LR
   control -> state -> [all modules]
   ```

3. Telemetry Flow:

   ```mermaid
    graph LR
   [all modules] -> telemetry -> storage -> control
   ```

4. Resource Management:

   ```mermaid
    graph LR
   [all modules] -> state (pressure signals)
   state -> [affected modules] (backpressure)
   ```

## Cross-Cutting Concerns

- Security (credentials, encryption)
- State management
- Telemetry
- Error handling
- Resource management
