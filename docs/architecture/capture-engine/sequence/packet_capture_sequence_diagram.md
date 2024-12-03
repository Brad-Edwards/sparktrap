# Packet Capture Sequence Diagram

```mermaid
sequenceDiagram
    box Data Path
    participant Input as Network Input
    participant Process as Packet Processing
    participant Output as Storage Output
    end
    box Core Management
    participant State as State Manager
    participant Buffer as Buffer Manager
    participant Security as Security Service
    end
    box Monitoring
    participant Monitor as Health/Telemetry
    end

    Note over Input,Monitor: Core Processing Flow with Security & State Management

    Security->>State: Initialize security policies
    State->>Input: Initialize capture state
    State->>Process: Configure processing/security rules
    State->>Output: Set output config
    
    loop Active Processing
        Input->>Security: Validate input source
        Input->>Buffer: Request buffer
        alt Buffer available and input validated
            Buffer-->>Input: Buffer granted
            Input->>Process: Raw packet batch
            
            alt Passes security checks & filters
                Process->>Security: Validate output destination
                Process->>Output: Filtered packets
                alt Output successful
                    Output->>Buffer: Release buffer
                else Output blocked
                    Output->>State: Signal backpressure
                    State->>Input: Adjust ingestion
                end
            else Security/Filter rejection
                Process->>Buffer: Release buffer
                Process->>Monitor: Log security event
                Security->>State: Update threat status
            end
            
        else Buffer pressure or security block
            Buffer->>State: Signal pressure
            State->>Input: Apply backpressure
        end
        
        Monitor->>State: Report health metrics
        Monitor->>Security: Report security metrics
        
        alt State/Security change needed
            Security->>State: Update security policies
            State->>Process: Update rules/config
            State->>Input: Update capture config
            State->>Output: Update output config
        end
    end

    Note over Input,Monitor: Security integrated at both service level and processing pipeline
