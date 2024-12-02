# Control Message Flow DFD

```mermaid
graph TD
    CP[Control Plane] -->|Commands| Control[Control<br/>Manager]
    Control -->|Config Updates| State((State<br/>Manager))
    
    State -->|Interface Config| Interface[Interface<br/>Manager]
    State -->|Capture Config| Capture[Capture<br/>Engine]
    State -->|Filter Rules| Filter[Filter<br/>Engine]
    State -->|Output Config| Output[Output<br/>Manager]
    
    Interface & Capture & Filter & Output -->|Status| Control
    Control -->|Responses| CP