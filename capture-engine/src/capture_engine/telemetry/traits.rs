// telemetry/traits.rs
/// `TelemetryManager` handles metrics and telemetry emission.
use async_trait::async_trait;
use std::collections::HashMap;

use crate::traits::{Error, HealthCheck, Lifecycle};

/// Trait for telemetry management.
#[async_trait]
pub trait TelemetryManager: Lifecycle + HealthCheck + Send + Sync {
    /// Collects a telemetry metric.
    fn collect_metric(&mut self, data: TelemetryData) -> Result<(), Error>;

    /// Reports collected metrics to external systems.
    async fn report_metrics(&self) -> Result<(), Error>;

    /// Exports metrics in a specified format.
    fn export_metrics(&self, format: ExportFormat) -> Result<Vec<u8>, Error>;
}

/// Represents telemetry data.
#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub timestamp: u64,
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<MetricUnit>,
    pub metric_type: MetricType,
    pub value: MetricValue,
    pub attributes: HashMap<String, String>,
    pub resource: Option<HashMap<String, String>>,
}

/// Standard metric types.
#[derive(Debug, Clone)]
pub enum MetricType {
    Counter,
    UpDownCounter,
    Gauge,
    Histogram,
}

/// Units for metrics.
#[derive(Debug, Clone)]
pub enum MetricUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    PacketsPerSecond,
    BytesPerSecond,
    Percent,
    Count,
}

/// Value of a metric.
#[derive(Debug, Clone)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
    Histogram {
        count: u64,
        sum: f64,
        buckets: Vec<(f64, u64)>,
    },
}

/// Formats for exporting metrics.
#[derive(Debug, Clone)]
pub enum ExportFormat {
    OpenTelemetry,
    Prometheus,
    JSON,
}
