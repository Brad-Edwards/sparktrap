mod managers {
    /// Manages memory allocation and lifecycle
    mod buffer_manager {
        // - Manages packet memory pools
        // - Provides buffer pools to capture pipeline
        // - Coordinates with storage_manager for NVMe overflow
        // - Reports pressure and stats to telemetry_manager
        // - Responds to memory pressure signals
    }

    /// Core capture pipeline orchestration
    mod capture_manager {
        // Primary data pipeline coordinator:
        // 1. Gets packets from interface_manager
        // 2. Obtains buffers from buffer_manager
        // 3. Sends to protocol_manager for header analysis
        // 4. Passes to filter_manager for decisions
        // 5. Routes accepted packets to protocol_manager for deep inspection
        // 6. Forwards to output_manager
        // - Reports pipeline stats to telemetry_manager
        // - Updates state via state_manager
    }

    /// Cloud integration 
    mod cloud_manager {
        // - Handles AWS instance lifecycle events
        // - Notifies state_manager of lifecycle changes
        // - Works with control_manager for graceful shutdown
        // - Manages AWS credentials via security_manager
        // - Reports cloud events to telemetry_manager
    }

    /// Control plane communication
    mod control_manager {
        // - Receives and distributes commands to appropriate managers
        // - Handles configuration updates
        // - Reports status via telemetry_manager
        // - Coordinates state changes through state_manager
    }

    /// Packet filtering
    mod filter_manager {
        // - Receives packets and header analysis from protocol_manager
        // - Applies filter rules
        // - Coordinates with interface_manager for hardware offload
        // - Reports filter stats to telemetry_manager
        // - Maintains filter state via state_manager
    }

    /// Network interface management
    mod interface_manager {
        // - Manages kernel bypass (DPDK/XDP)
        // - Handles packet reception
        // - Provides packets to capture_manager
        // - Reports interface stats to telemetry_manager
        // - Maintains interface state via state_manager
    }

    /// Data output handling
    mod output_manager {
        // - Receives processed packets from capture_manager
        // - Uses storage_manager for buffering if needed
        // - Manages data transmission off engine
        // - Reports output stats to telemetry_manager
        // - Maintains output state via state_manager
    }

    /// Protocol analysis
    mod protocol_manager {
        // Two distinct responsibilities:
        // 1. Headers:
        //    - Fast L2-L4 parsing before filtering
        //    - Provides header info to filter_manager
        // 2. Deep:
        //    - Full protocol analysis post-filtering
        // - Reports protocol stats to telemetry_manager
        // - Maintains protocol state via state_manager
    }

    /// Security management
    mod security_manager {
        // - Manages credentials and security policies
        // - Coordinates with cloud_manager for AWS auth
        // - Reports security events to telemetry_manager
        // - Maintains security state via state_manager
    }

    /// State coordination
    mod state_manager {
        // - Maintains state for all managers
        // - Coordinates state transitions
        // - Uses storage_manager for persistence
        // - Reports state changes to telemetry_manager
    }

    /// Local NVMe management
    mod storage_manager {
        // - Manages local NVMe resources
        // - Provides buffering for buffer_manager
        // - Handles storage pressure
        // - Reports storage stats to telemetry_manager
        // - Maintains storage state via state_manager
    }

    /// Metrics and monitoring
    mod telemetry_manager {
        // - Collects metrics from all managers
        // - Manages logging infrastructure
        // - Reports system health
        // - Uses storage_manager for persistence
    }
}