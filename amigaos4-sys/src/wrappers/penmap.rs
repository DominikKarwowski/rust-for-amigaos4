//! IPenMap global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `PENMAP_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::penmap::*;

// ---- IPenMap (PenMapIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IPenMap: *mut PenMapIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IPenMap: *mut PenMapIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn penmap_penmap_get_class() -> *mut APTR {
    ((*IPenMap).PENMAP_GetClass)(IPenMap)
}
