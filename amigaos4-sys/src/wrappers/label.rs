//! ILabel global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `LABEL_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::label::*;

// ---- ILabel (LabelIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ILabel: *mut LabelIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ILabel: *mut LabelIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn label_label_get_class() -> *mut APTR {
    ((*ILabel).LABEL_GetClass)(ILabel)
}
