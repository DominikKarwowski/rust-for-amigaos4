//! IInput global(s) and convenience wrappers.
//!
//! Hand-written binding for input.library. Tiny — exposes the
//! currently-active qualifier mask (shift / ctrl / etc.).

use crate::types::*;
use crate::interfaces::input::*;

// ---- IInput (InputIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IInput: *mut InputIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IInput: *mut InputIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn input_peek_qualifier() -> u16 {
    ((*IInput).PeekQualifier)(IInput)
}
