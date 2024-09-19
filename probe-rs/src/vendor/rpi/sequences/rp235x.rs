//! Sequences for RP235x devices

use std::sync::Arc;

use probe_rs_target::CoreType;

use crate::architecture::arm::{
    component::{TraceFunnel, TraceSink},
    memory::{romtable::RomTableError, ArmMemoryInterface, CoresightComponent, PeripheralType},
    sequences::ArmDebugSequence,
    ArmError, ArmProbeInterface, FullyQualifiedApAddress,
};

/// Marker struct indicating initialization sequencing for RP235x family parts.
#[derive(Debug)]
pub struct Rp235x {}

impl Rp235x {
    /// Create the sequencer for the H7 family of parts.
    pub fn create() -> Arc<Self> {
        tracing::info!("new Rp235x sequence");
        Arc::new(Self {})
    }
}

impl ArmDebugSequence for Rp235x {}
