//! IInteger global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `INTEGER_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::integer::*;

// ---- IInteger (IntegerIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IInteger: *mut IntegerIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IInteger: *mut IntegerIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn integer_integer_get_class() -> *mut APTR {
    ((*IInteger).INTEGER_GetClass)(IInteger)
}
