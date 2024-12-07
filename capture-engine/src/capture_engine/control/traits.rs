// control/traits.rs
use crate::traits::{Error, EventHandler, HealthCheck, Lifecycle};
/// `ControlManager` handles control events like configuration updates or start/stop commands.
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;

/// Events specific to control management.
#[derive(Debug)]
pub enum ControlEvent {
    ConfigurationUpdate(Configuration),
    Command(ControlCommand),
}

/// Control commands from the control plane.
#[derive(Debug)]
pub enum ControlCommand {
    StartCapture,
    StopCapture,
    UpdateFilters(FilterConfig),
    Pause,
    Resume,
}

/// Control manager trait.
#[async_trait]
pub trait ControlManager:
    Lifecycle + EventHandler<ControlEvent> + HealthCheck + Send + Sync
{
    /// Sends a status update to the control plane.
    async fn send_status(&self) -> Result<(), Error>;

    /// Applies a configuration update.
    async fn apply_configuration(&mut self, config: Configuration) -> Result<(), Error>;
}

/// Represents the configuration data.
#[derive(Debug, Clone)]
pub struct Configuration {
    pub settings: HashMap<String, String>,
}

/// Filter configuration.
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub rules: Vec<FilterRule>,
    pub default_action: FilterAction,
}

/// Filter rule for packet filtering.
#[derive(Debug, Clone)]
pub struct FilterRule {
    pub id: String,
    pub priority: u32,
    pub conditions: Vec<FilterCondition>,
    pub action: FilterAction,
}

/// Conditions for a filter rule.
#[derive(Debug, Clone)]
pub enum FilterCondition {
    SourceIp(IpAddr),
    DestIp(IpAddr),
    SourcePort(u16),
    DestPort(u16),
    Protocol(u8),
}

/// Actions for filter rules.
#[derive(Debug, Clone)]
pub enum FilterAction {
    Accept,
    Drop,
    Mirror,
}
