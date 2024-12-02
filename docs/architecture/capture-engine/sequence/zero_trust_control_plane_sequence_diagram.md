# Zero Trust Control Plane Sequence Diagram

```mermaid
sequenceDiagram
    box External
    participant CP as Control Plane
    end
    box Control Layer
    participant Auth as Auth Service
    participant CM as Control Manager
    participant SM as Security Manager
    participant State as State Manager
    end
    box Components
    participant Interface as Interface Manager
    participant Capture as Capture Engine
    participant Filter as Filter Engine
    participant Output as Output Manager
    end

    Note over CP,Output: Zero Trust Control Flow with Security Logging

    CP->>Auth: Request operation token
    Auth->>Output: Log auth attempt
    alt Auth successful
        Auth->>CP: Issue limited-time token
        Auth->>Output: Log token issued
    else Auth failed
        Auth->>Output: Log auth failure + alert
        Auth->>CP: Auth rejected
    end
    
    CP->>CM: Submit config update + token
    CM->>Auth: Validate token & permissions
    CM->>Output: Log validation attempt
    CM->>SM: Validate config change + token
    
    alt Token & permissions valid
        SM->>Auth: Verify component access rights
        SM->>Output: Log access check
        Auth->>SM: Access confirmation
        SM->>State: Apply validated config + token
        
        par Component Updates with Auth
            State->>Auth: Get component token
            Auth->>Output: Log token request
            Auth->>State: Issue component token
            
            State->>Interface: Update config + token
            Interface->>Auth: Verify token
            Interface->>Output: Log config attempt
            Interface-->>State: Ack/Reject change
            
            State->>Capture: Update config + token
            Capture->>Auth: Verify token
            Capture->>Output: Log config attempt
            Capture-->>State: Ack/Reject change
            
            State->>Filter: Update rules + token
            Filter->>Auth: Verify token
            Filter->>Output: Log config attempt
            Filter-->>State: Ack/Reject change
            
            State->>Output: Update config + token
            Output->>Auth: Verify token
            Output->>Output: Log config attempt
            Output-->>State: Ack/Reject change
        end

        alt All components accepted
            State->>CM: Config applied
            State->>Output: Log successful update
            CM->>CP: Update successful
        else Component rejection
            State->>State: Rollback changes
            State->>Output: Log rollback + alert
            State->>CM: Update failed
            CM->>CP: Update failed with details
        end

    else Auth/Security validation fails
        Auth->>Output: Log security violation + alert
        Auth->>CM: Auth rejection
        CM->>CP: Update rejected (unauthorized)
    end

    loop Continuous Status (with auth)
        Interface->>Auth: Refresh status token
        Interface->>Output: Log token refresh
        Interface->>State: Report health + token
        
        Capture->>Auth: Refresh status token
        Capture->>Output: Log token refresh
        Capture->>State: Report health + token
        
        Filter->>Auth: Refresh status token
        Filter->>Output: Log token refresh
        Filter->>State: Report health + token
        
        Output->>Auth: Refresh status token
        Output->>Output: Log token refresh
        Output->>State: Report health + token
        
        State->>Auth: Refresh aggregation token
        State->>Output: Log aggregation
        State->>CM: Aggregate status + token
        CM->>Auth: Verify token
        CM->>CP: System status update
    end

    loop Token Refresh (periodic)
        Auth->>CP: Request credential refresh
        Auth->>Output: Log refresh request
        CP->>Auth: Update credentials
        Auth->>Output: Log credential update
        Auth->>Auth: Rotate component tokens
        Auth->>Output: Log token rotation
    end

    Note over CP,Output: Every security event logged, alerts for critical issues