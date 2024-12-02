# Interface States Diagram

```mermaid
stateDiagram-v2
    [*] --> Initializing
    
    Initializing --> Ready: Kernel Bypass Setup Complete
    Initializing --> Failed: Setup Error
    
    Ready --> Receiving: Traffic Started
    Ready --> Down: Network Error
    
    Receiving --> Ready: No Traffic
    Receiving --> Down: Network Error
    
    Down --> Ready: Network Restored
    Down --> Failed: Permanent Error
    
    Failed --> [*]: Cleanup Complete