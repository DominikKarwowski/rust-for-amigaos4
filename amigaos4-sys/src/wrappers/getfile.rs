//! IGetFile global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `GETFILE_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::getfile::*;

// ---- IGetFile (GetFileIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IGetFile: *mut GetFileIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IGetFile: *mut GetFileIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn getfile_getfile_get_class() -> *mut APTR {
    ((*IGetFile).GETFILE_GetClass)(IGetFile)
}
