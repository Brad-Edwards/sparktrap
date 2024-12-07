// capture/trait.rs
use crate::traits::{
    BackpressureControl, Cleanup, Error, HealthCheck, Lifecycle, Packet, PauseResume,
    PressureAction, PressureAware, PressureStatus, RateLimiter, ResourceManager, StartStop,
};
/// The `CaptureManager` orchestrates the packet pipeline from ingestion to output.
///
/// Although scaling is handled elsewhere, `CaptureManager` still needs to handle local pressure
/// and adapt by throttling or applying backpressure.
use async_trait::async_trait;

/// Trait for managing the capture process.
#[async_trait]
pub trait CaptureManager:
    Lifecycle
    + StartStop
    + PauseResume
    + HealthCheck
    + PressureAware
    + RateLimiter
    + BackpressureControl
    + ResourceManager
    + Cleanup
    + Send
    + Sync
{
    async fn receive_batch(&mut self, packets: *mut Packet, count: usize) -> Result<(), Error>;
    fn pipeline_pressure_status(&self) -> PipelinePressure;
    async fn handle_stage_backpressure(
        &mut self,
        stage: PipelineStage,
        action: PressureAction,
    ) -> Result<(), Error>;
}

// A fixed structure for pipeline stages:
pub struct PipelinePressure {
    pub ingestion: PressureStatus,
    pub light_parse: PressureStatus,
    pub deep_parse: PressureStatus,
    pub filtering: PressureStatus,
    pub output: PressureStatus,
}

/// Different stages in the capture processing pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineStage {
    Ingestion,
    LightParse,
    DeepParse,
    Filtering,
    Output,
}
