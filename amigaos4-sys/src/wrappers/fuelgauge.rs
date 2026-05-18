//! IFuelGauge global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `FUELGAUGE_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::fuelgauge::*;

// ---- IFuelGauge (FuelGaugeIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IFuelGauge: *mut FuelGaugeIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IFuelGauge: *mut FuelGaugeIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn fuelgauge_fuelgauge_get_class() -> *mut APTR {
    ((*IFuelGauge).FUELGAUGE_GetClass)(IFuelGauge)
}
