//! Raspberry Pi vendor support.

use probe_rs_target::Chip;

use crate::{
    config::DebugSequence,
    vendor::{rpi::sequences::rp235x::Rp235x, Vendor},
};

pub mod sequences;

/// Texas Instruments
#[derive(docsplay::Display)]
pub struct RaspberryPi;

impl Vendor for RaspberryPi {
    fn try_create_debug_sequence(&self, chip: &Chip) -> Option<DebugSequence> {
        let sequence = if chip.name.starts_with("RP235") {
            DebugSequence::Arm(Rp235x::create())
        } else {
            return None;
        };

        Some(sequence)
    }
}
