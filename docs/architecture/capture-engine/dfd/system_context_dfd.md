# System Context DFD

```mermaid
graph LR
    subgraph External Sources
        VPC[AWS VPC<br/>Traffic Mirror]
        Control[Control Plane]
        Auth[AWS IAM/<br/>Security Services]
    end

    subgraph External Consumers
        Storage[(Long-term<br/>Storage)]
        Monitor[Monitoring/<br/>SIEM Systems]
        Analysis[Security Tools<br/>Zeek/Suricata]
    end

    System((Cloud-Native<br/>Packet Capture<br/>System))

    VPC -->|Mirrored Traffic| System
    Control -->|Commands/Config/Updates| System
    Auth -->|Credentials/Policies| System
    
    System -->|Captured Packets| Storage
    System -->|Metrics/Telemetry/Alerts| Monitor
    System -->|Real-time Feed| Analysis
    System -->|Status/Health/Logs| Control