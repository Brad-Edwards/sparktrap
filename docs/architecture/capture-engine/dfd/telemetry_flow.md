# Telemetry Flow DFD

graph TD
    Interface[Interface<br/>Manager] -->|Metrics| Telemetry[Telemetry<br/>Collector]
    Capture[Capture<br/>Engine] -->|Stats| Telemetry
    Protocol[Protocol<br/>Analysis] -->|Stats| Telemetry
    Filter[Filter<br/>Engine] -->|Counts| Telemetry
    Output[Output<br/>Manager] -->|Metrics| Telemetry
    Buffer[Buffer<br/>Manager] -->|Usage| Telemetry
    
    Telemetry -->|Metrics| Monitor[Monitoring<br/>Systems]
    Telemetry -->|Alerts| SIEM[SIEM]