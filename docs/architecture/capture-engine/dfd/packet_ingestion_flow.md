# Packet Ingestion Flow DFD

```mermaid
graph LR
    VPC[VPC Mirror] -->|Mirror Traffic| Interface[Kernel Bypass<br/>Interface]
    Interface -->|Raw Packets| Capture[Capture<br/>Engine]
    
    Capture -->|Packets| PLite[Protocol<br/>Light Parse]
    PLite -->|Basic Headers| Filter1[Initial<br/>Filter]
    
    Filter1 -->|Accepted| PDeep[Protocol<br/>Deep Analysis]
    PDeep -->|Full Analysis| Filter2[Secondary<br/>Filter]
    
    Filter2 -->|Accepted Packets| Output[Output<br/>Manager]
    Output -->|Stored Packets| Storage[(Storage)]
    
    State((State<br/>Manager)) -.->|State Updates| Interface & Capture & Filter1 & Filter2 & Output
    Buffer((Buffer<br/>Manager)) -.->|Memory Management| Interface & Capture & Output