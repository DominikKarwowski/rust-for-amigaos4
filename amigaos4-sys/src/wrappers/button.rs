//! IButton global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `BUTTON_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::button::*;

// ---- IButton (ButtonIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IButton: *mut ButtonIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IButton: *mut ButtonIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn button_button_get_class() -> *mut APTR {
    ((*IButton).BUTTON_GetClass)(IButton)
}
