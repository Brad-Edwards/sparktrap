# Capture Session State Diagram

```mermaid
stateDiagram-v2
    [*] --> Initializing
    
    Initializing --> Ready: Resources Allocated
    Initializing --> Failed: Resource Error
    
    Ready --> Capturing: Start Command
    Ready --> ShuttingDown: Stop Command
    
    Capturing --> Paused: Pause Command/Buffer Full
    Capturing --> Error: Capture Error
    Capturing --> ShuttingDown: Stop Command
    
    Paused --> Capturing: Resume Command/Buffer Space
    Paused --> ShuttingDown: Stop Command
    
    Error --> Ready: Error Cleared
    Error --> ShuttingDown: Stop Command
    
    ShuttingDown --> [*]: Cleanup Complete
    Failed --> [*]: Cleanup Complete