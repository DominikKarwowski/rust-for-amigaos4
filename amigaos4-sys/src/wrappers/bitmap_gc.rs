//! IBitMap global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `BITMAP_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::bitmap_gc::*;

// ---- IBitMap (BitMapIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IBitMap: *mut BitMapIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IBitMap: *mut BitMapIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn bitmap_gc_bitmap_get_class() -> *mut APTR {
    ((*IBitMap).BITMAP_GetClass)(IBitMap)
}
