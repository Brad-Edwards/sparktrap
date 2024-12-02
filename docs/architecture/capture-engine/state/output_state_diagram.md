# Output State Diagram

```mermaid
stateDiagram-v2
    [*] --> Initializing
    
    Initializing --> Ready: Systems Connected
    Initializing --> Failed: Init Error
    
    Ready --> Processing: Data Available
    
    Processing --> Ready: Queue Empty
    Processing --> Blocked: Resource Limit
    
    state Processing {
        [*] --> Routing
        Routing --> StorageWrite: Storage Output
        Routing --> TelemetryWrite: Telemetry Output
        Routing --> ControlWrite: Control Response
        
        StorageWrite --> Routing
        TelemetryWrite --> Routing
        ControlWrite --> Routing
    }
    
    Blocked --> Processing: Resources Available
    Blocked --> Failed: Fatal Error
    
    Failed --> [*]: Cleanup Complete