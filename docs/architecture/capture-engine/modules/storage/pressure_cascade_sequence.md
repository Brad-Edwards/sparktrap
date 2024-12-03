# Pressure Cascade Sequence Diagram

```mermaid
sequenceDiagram
    participant Int as Interface
    participant NVMe
    participant NVMeMgr as NVMe Manager
    participant BufMgr as Buffer Manager
    participant StgMgr as Storage Manager
    participant Cap as Capture Module
    participant Tel as Telemetry
    
    Note over Int,Tel: Pressure Detection & Cascade

    Note over Int,Tel: Stage 1: Initial Pressure
    NVMe-->>NVMeMgr: Queue Depth > 70%
    activate NVMeMgr
    NVMeMgr-->>Tel: Storage Pressure Alert
    NVMeMgr->>StgMgr: Storage Pressure Signal
    StgMgr->>BufMgr: Adjust Write Rate
    BufMgr->>Cap: Slow Ingestion Rate
    Cap->>Int: Reduce Packet Rate
    Int-->>Cap: Rate Adjusted
    Cap-->>BufMgr: Acknowledge Rate Change
    BufMgr-->>Tel: Rate Adjustment Metrics
    deactivate NVMeMgr

    Note over Int,Tel: Stage 2: Increasing Pressure
    NVMe-->>NVMeMgr: Queue Depth > 85%
    activate StgMgr
    NVMeMgr->>StgMgr: Critical Storage Pressure
    StgMgr->>BufMgr: Reduce Buffer Allocation
    BufMgr->>Cap: Reduce Capture Rate
    Cap->>Int: Adjust Ring Parameters
    Int->>Int: Throttle Input
    Int-->>Cap: Input Throttled
    Cap-->>BufMgr: Buffer Adjusted
    BufMgr-->>StgMgr: Backpressure Applied
    Cap-->>Tel: Capture Rate Metrics
    deactivate StgMgr

    Note over Int,Tel: Stage 3: Critical Pressure
    BufMgr-->>StgMgr: Buffer Utilization > 95%
    activate StgMgr
    StgMgr-->>Tel: Critical System Pressure
    StgMgr->>Cap: Enter Degraded Mode
    Cap->>Int: Enable Drop Policy
    Int->>Int: Apply Packet Filtering
    Cap->>BufMgr: Reduce Buffer Consumption
    BufMgr->>NVMeMgr: Slow Write Rate
    NVMeMgr->>NVMe: Adjust Queue Depth
    Cap-->>Tel: Degraded Mode Metrics
    deactivate StgMgr

    Note over Int,Tel: Stage 4: Recovery
    NVMe-->>NVMeMgr: Queue Depth < 60%
    activate NVMeMgr
    NVMeMgr->>StgMgr: Pressure Relieved
    StgMgr->>BufMgr: Restore Normal Rates
    BufMgr->>Cap: Resume Normal Operation
    Cap->>Int: Reset Packet Policy
    Int->>Int: Reset Ring Parameters
    Int-->>Cap: Normal Operation Resumed
    Cap-->>BufMgr: Normal Operation Resumed
    BufMgr-->>NVMeMgr: Normal Write Rate
    Cap-->>Tel: Recovery Metrics
    deactivate NVMeMgr

    Note over Int,Tel: Continuous Monitoring & Adjustment