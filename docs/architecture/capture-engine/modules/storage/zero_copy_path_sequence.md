# Zero Copy Path Sequence Diagram

```mermaid
sequenceDiagram
    participant NIC
    participant BufInt as Buffer Interface
    participant BufMgr as Buffer Manager
    participant NVMeMgr as NVMe Manager
    participant NVMe
    participant Tel as Telemetry
    
    Note over NIC,NVMe: Initialization Phase
    
    BufMgr->>BufMgr: Calculate Ring Buffer Sizes
    BufMgr->>NVMeMgr: Initialize Queue Pairs
    NVMeMgr->>NVMe: Setup Submission/Completion Queues
    NVMeMgr-->>Tel: Queue Setup Metrics
    
    BufMgr->>BufInt: Allocate Huge Pages
    BufInt->>BufInt: Setup DMA Ring Buffers
    BufInt->>NIC: Register DMA Mappings
    BufInt-->>Tel: DMA Map Metrics
    
    NVMeMgr->>NVMe: Configure BAR Memory Windows
    NVMeMgr->>NVMe: Verify Direct Access
    NVMeMgr-->>Tel: NVMe Config Metrics
    
    Note over NIC,NVMe: Zero-Copy Operation Phase
    
    loop Continuous Operation
        NIC-->>BufInt: DMA Write to Ring Buffer
        activate BufInt
        
        BufInt-->>BufMgr: Update Buffer Head
        BufMgr->>BufMgr: Check Pressure Thresholds
        
        alt Normal Pressure
            BufMgr-->>NVMeMgr: Batch Write Ready
            NVMeMgr-->>NVMe: DMA Transfer (Zero-Copy)
            NVMeMgr-->>Tel: Write Performance Metrics
        else High Pressure
            BufMgr-->>Tel: Pressure Alert
            BufMgr->>BufMgr: Apply Backpressure
        end
        
        NVMe-->>NVMeMgr: Completion Signal
        NVMeMgr-->>BufMgr: Update Write Position
        deactivate BufInt
        
        Note over BufInt,BufMgr: Buffer Management
        BufMgr->>BufMgr: Check Free Space
        BufMgr->>BufInt: Release Written Buffers
        BufInt->>BufMgr: Update Free Pool
        BufMgr-->>Tel: Buffer Metrics
    end
    
    Note over NIC,NVMe: Error Handling
    
    alt NVMe Error
        NVMe-->>NVMeMgr: Error Signal
        NVMeMgr-->>Tel: Error Metrics
        NVMeMgr->>NVMeMgr: Error Recovery Procedure
    else Buffer Overflow Risk
        BufMgr-->>Tel: Critical Pressure Alert
        BufMgr->>BufInt: Emergency Buffer Release
    end