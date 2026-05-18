//! ICheckBox global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `CHECKBOX_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::checkbox::*;

// ---- ICheckBox (CheckBoxIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICheckBox: *mut CheckBoxIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICheckBox: *mut CheckBoxIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn checkbox_checkbox_get_class() -> *mut APTR {
    ((*ICheckBox).CHECKBOX_GetClass)(ICheckBox)
}
