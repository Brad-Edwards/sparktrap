// event/traits.rs
use crate::capture_engine::buffer::traits::BufferEvent;
use crate::traits::{Error, PressureStatus};
/// `EventSystem` defines a generic system for event publication and subscription.
///
/// Downstream submodules can plug into the appropriate message queues or streaming services.
use tokio::sync::mpsc;

/// Events that can occur in the system.
#[derive(Debug)]
pub enum SystemEvent {
    BufferEvent(BufferEvent),
    CaptureEvent,
    CloudEvent,
    ControlEvent,
    FilterEvent,
    InterfaceEvent,
    OutputEvent,
    ProtocolEvent,
    SecurityEvent,
    StateEvent,
    StorageEvent,
    TelemetryEvent,
    PressureEvent(PressureStatus),
    ResourceEvent,
    LifecycleEvent,
    ErrorEvent(Error),
    CustomEvent(String),
}

/// Metadata for events.
#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub id: String,
    pub timestamp: u64,
    pub priority: EventPriority,
    pub correlation_id: Option<String>,
    pub source: String,
}

/// Priority levels for events.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Critical,
    High,
    Normal,
    Low,
}

/// Represents an event in the system.
#[derive(Debug)]
pub struct Event {
    pub metadata: EventMetadata,
    pub payload: SystemEvent,
}

/// The event system for publishing and subscribing to events.
pub struct EventSystem {}

impl EventSystem {
    /// Publishes an event to the system.
    pub async fn publish(&self, _event: Event) -> Result<(), Error> {
        Ok(())
    }

    /// Subscribes to events based on filters.
    pub fn subscribe(&self, _filters: Vec<EventFilter>) -> mpsc::Receiver<Event> {
        // TODO: Implement
        mpsc::channel(100).1
    }
}

pub enum EventFilter {
    ByType(SystemEventType),
    ByPriority(EventPriority),
    BySource(String),
    Custom(Box<dyn Fn(&Event) -> bool + Send + Sync>),
}

// Manual Clone implementation
impl Clone for EventFilter {
    fn clone(&self) -> Self {
        match self {
            EventFilter::ByType(t) => EventFilter::ByType(t.clone()),
            EventFilter::ByPriority(p) => EventFilter::ByPriority(p.clone()),
            EventFilter::BySource(s) => EventFilter::BySource(s.clone()),
            EventFilter::Custom(_) => panic!("Custom filters cannot be cloned"),
            // Alternative: return a no-op filter instead of panicking
            // EventFilter::Custom(Box::new(|_| true))
        }
    }
}

/// Represents types of system events.
#[derive(Debug, Clone)]
pub enum SystemEventType {
    BufferEvent,
    CaptureEvent,
    CloudEvent,
    ControlEvent,
    FilterEvent,
    InterfaceEvent,
    OutputEvent,
    ProtocolEvent,
    SecurityEvent,
    StateEvent,
    StorageEvent,
    TelemetryEvent,
    PressureEvent,
    ResourceEvent,
    LifecycleEvent,
    ErrorEvent,
    CustomEvent,
}
