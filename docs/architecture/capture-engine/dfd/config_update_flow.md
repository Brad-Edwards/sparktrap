# Configuration Update Flow DFD

```mermaid
graph TD
    CP[Control Plane] -->|Config Update| Control[Control<br/>Manager]
    Control -->|Validate| Security[Security<br/>Manager]
    Security -->|Authorized| State((State<br/>Manager))
    
    State -->|Apply| Components[Component<br/>Managers]
    Components -->|Status| State
    State -->|Result| Control
    Control -->|Response| CP