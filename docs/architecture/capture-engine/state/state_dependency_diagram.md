# Capture Engine State Dependency Diagram

```mermaid
graph TD
    control[Control<br/>orchestration] --> capture[Capture<br/>core]
    control --> cloud[Cloud<br/>lifecycle]
    control --> interface[Interface<br/>network]
    
    cloud --> security[Security<br/>creds/auth]
    
    capture --> buffer[Buffer<br/>memory]
    capture --> interface
    capture --> protocol[Protocol<br/>analysis]
    capture --> filter[Filter<br/>rules]
    capture --> output[Output<br/>storage]
    
    protocol --> filter
    protocol --> output
    
    filter --> output
    
    buffer --> output
    
    %% All modules report to telemetry but omitted for clarity
    
    classDef default fill:#f8f9fa,stroke:#2f2f2f,stroke-width:1px,color:#000;
    classDef core fill:#e9ecef,stroke:#2f2f2f,stroke-width:2px,color:#000;
    
    class control,capture core;