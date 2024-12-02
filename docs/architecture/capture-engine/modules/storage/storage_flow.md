# Storage DFD

```mermaid
graph LR
    %% External Systems & Interfaces
    Buffer[Buffer Module] -->|Zero Copy Path| BufInt[Buffer Interface]
    Buffer -->|Memory Management| BufInt
    State[State Module] <-->|State Sync| StgMgr[Storage Manager]
    Security[Security] <-->|Encryption/Auth| StgMgr
    
    %% Core Management & Control
    StgMgr -->|Policy/Quotas| BufInt & NVMe & IO & Life & State_Idx
    StgMgr -->|Orchestration| NVMe & IO & Life & State_Idx
    
    %% Critical Performance Path
    BufInt -->|Direct NVMe Access| NVMe[NVMe Manager]
    BufInt -->|Non-Critical Data| IO[I/O Pipeline]
    
    %% Data Paths
    IO -->|Compressed Data| NVMe
    IO -->|State Data| State_Idx[State & Index Manager]
    State_Idx <-->|State Sync| State
    
    %% Lifecycle and Cleanup
    Life[Lifecycle Manager] -->|Retention/Cleanup| NVMe
    Life -->|Index Cleanup| State_Idx
    Life <-->|Security Policy| Security
    
    %% Monitoring
    Tel((Telemetry)) -.->|Metrics| NVMe & IO & Life & State_Idx & BufInt & StgMgr
    
    %% AWS Integration
    AWS[AWS Services] -->|KMS| Security
    AWS -->|Backup/CloudWatch| StgMgr
    
    %% Pressure Signaling Mesh
    NVMe -->|Storage Pressure| BufInt & StgMgr
    IO -->|Pipeline Pressure| StgMgr
    BufInt -->|Memory Pressure| StgMgr
    
    classDef default fill:#ffffff,stroke:#000000,stroke-width:2px,color:#000000;
    classDef external fill:#ffffff,stroke:#000000,stroke-width:2px,color:#000000;
    classDef monitor fill:#ffffff,stroke:#000000,stroke-width:2px,color:#000000;
    classDef critical fill:#ffffff,stroke:#000000,stroke-width:4px,color:#000000;
    
    class Buffer,State,AWS,Security external;
    class Tel monitor;
    class NVMe,BufInt critical;