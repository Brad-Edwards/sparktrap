# Storage State Diagram

```mermaid
stateDiagram-v2
    [*] --> INIT
    INIT --> NORMAL: System Ready
    
    state INIT {
        [*] --> ResourceCheck
        ResourceCheck --> ConfigValidation
        ConfigValidation --> SecurityInit
        SecurityInit --> WarmUp
        
        state ResourceCheck {
            [*] --> NVMeValidation
            NVMeValidation --> MemoryAllocation
            MemoryAllocation --> NetworkInit
        }
        
        state SecurityInit {
            [*] --> KMSCheck
            KMSCheck --> CredsValidation
            CredsValidation --> EncryptionInit
        }
        
        state WarmUp {
            [*] --> BufferPrealloc
            BufferPrealloc --> IndexPrep
            IndexPrep --> ReadyState
        }
    }

    state NORMAL {
        [*] --> OptimalFlow
        
        state OptimalFlow {
            [*] --> ZeroCopyActive
            ZeroCopyActive --> CompressActive
            CompressActive --> IndexActive
            
            state ZeroCopyActive {
                DirectMemory
                NVMeDirect
                BufferManaged
            }
        }
        
        state MonitoringActive {
            PressureWatch
            MetricsCollection
            HealthCheck
        }
    }

    state "PRESSURE (By Source)" as PRESSURE {
        [*] --> MemoryPressure
        [*] --> StoragePressure
        [*] --> PipelinePressure
        
        state MemoryPressure {
            state BufferControl {
                AdjustAllocation
                ThrottleIngestion
                ForceClear
            }
        }
        
        state StoragePressure {
            state NVMeControl {
                AdjustWriteRate
                IncreaseDedupe
                ForceFlush
            }
        }
        
        state PipelinePressure {
            state QueueControl {
                AdjustBatch
                ReorderQueues
                ScaleThreads
            }
        }
    }

    state DEGRADED {
        [*] --> PartialCapture
        
        state PartialCapture {
            EvaluateFlows
            PrioritizeCritical
            SuspendNonCritical
        }
        
        state DegradedMonitoring {
            ResourceTracking
            RecoveryPlanning
            AlertEscalation
        }
    }

    state CRITICAL {
        [*] --> EmergencyMode
        
        state EmergencyMode {
            SaveState
            SecureData
            NotifyControl
        }
        
        state FailoverPrep {
            StateDump
            BufferFlush
            IndexSync
        }
    }

    state AWS_LIFECYCLE {
        state InstanceStates {
            Launching
            Running
            Stopping
            Terminated
        }
        
        state HealthStates {
            Healthy
            Impaired
            InsufficientData
        }
        
        state ScalingStates {
            ScalePrep
            ScaleExec
            ScaleComplete
        }
    }

    state MAINTENANCE {
        state Backup {
            SnapshotInit
            SecureTransfer
            VerifyIntegrity
        }
        
        state Cleanup {
            RetentionCheck
            SecureDelete
            IndexCleanup
        }
        
        state Recovery {
            StateRestore
            IndexRebuild
            IntegrityVerify
        }
        
        state ComplianceOps {
            AuditPrep
            LogArchival
            ComplianceCheck
        }
    }

    NORMAL --> PRESSURE: Source-Specific Threshold
    PRESSURE --> NORMAL: Resource Recovery
    PRESSURE --> DEGRADED: Sustained Pressure
    DEGRADED --> PRESSURE: Partial Recovery
    DEGRADED --> CRITICAL: Resource Exhaustion
    CRITICAL --> DEGRADED: Emergency Recovery
    
    any --> AWS_LIFECYCLE: Cloud Event
    AWS_LIFECYCLE --> INIT: Instance Start
    AWS_LIFECYCLE --> CRITICAL: Instance Stop/Terminate
    
    any --> MAINTENANCE: Scheduled/Manual
    MAINTENANCE --> INIT: Tasks Complete

    note right of INIT
        Complete initialization sequence
        Resource & security validation
        Warm-up procedures
    end note

    note right of NORMAL
        Zero-copy path active
        Full monitoring
        Optimal performance
    end note

    note right of PRESSURE
        Granular pressure handling
        Source-specific responses
        Performance optimization
    end note