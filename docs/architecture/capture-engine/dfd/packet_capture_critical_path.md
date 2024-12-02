# Packet Capture Critical Path DFD

```mermaid
graph TD
    %% AWS VPC Mirror Input
    VPC[VPC Mirror] -->|Mirror Session| ENI[Elastic Network<br/>Interface]
    ENI -->|Network I/O| KBypass[Kernel Bypass<br/>Network Stack]
    
    subgraph "Capture Processing"
        KBypass -->|Packets| Validate[Packet<br/>Validation]
        Validate -->|Verified| MemPool[Memory<br/>Pool]
        MemPool -->|Allocated| PLite[Protocol<br/>Light Parse]
        
        PLite -->|Basic Headers| Filter[Initial<br/>Filter]
        PLite -->|Full Packet| Hold[Packet<br/>Hold Queue]
        
        Filter -->|Decision| Route{Route}
        Hold -->|Await| Route
        
        Route -->|Reject| Free[Memory<br/>Release]
        Route -->|Accept| PDeep[Protocol<br/>Deep Analysis]
        
        PDeep -->|Analysis| SecFilter[Secondary<br/>Filter]
        SecFilter -->|Accept| Batch[Output<br/>Batcher]
        SecFilter -->|Reject| Free
    end
    
    %% Output Processing
    Batch -->|Full Buffer| Write[Write<br/>Manager]
    Write -->|Local Buffer| NVMe[(NVMe<br/>Buffer)]
    Write -->|Chunks| S3[(S3/Storage)]
    
    %% State & Control
    State((State<br/>Manager)) -.->|Config| Filter & SecFilter & Write
    SecMon((Security<br/>Monitor)) -.->|Anomaly Detection| Validate & PLite & PDeep
    Monitor((Telemetry)) -.->|Stats| KBypass & MemPool & Filter & PDeep & Write
    
    %% Resource Management
    Free -->|Released| MemPool
    Write -->|Complete| Free
