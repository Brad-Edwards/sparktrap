# Filter State Diagram

```mermaid
stateDiagram-v2
    [*] --> Loading
    
    Loading --> Active: Rules Loaded
    Loading --> Failed: Rule Load Error
    
    Active --> Filtering: Traffic Present
    Active --> Updating: Rule Update
    
    Filtering --> Active: No Traffic
    Filtering --> Updating: Rule Update
    
    Updating --> Active: Update Complete
    Updating --> Failed: Update Error
    
    Failed --> Active: Error Resolved
    Failed --> [*]: Fatal Error