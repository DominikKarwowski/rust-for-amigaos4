//! ISpace global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `SPACE_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::space::*;

// ---- ISpace (SpaceIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ISpace: *mut SpaceIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ISpace: *mut SpaceIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn space_space_get_class() -> *mut APTR {
    ((*ISpace).SPACE_GetClass)(ISpace)
}
