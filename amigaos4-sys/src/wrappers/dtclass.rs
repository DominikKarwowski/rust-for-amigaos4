//! IDTClass global(s) and convenience wrappers.
//!
//! Hand-written binding for datatypes-class engine access. One
//! method that returns the engine pointer the class subsystem uses.

use crate::types::*;
use crate::interfaces::dtclass::*;

// ---- IDTClass (DTClassIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IDTClass: *mut DTClassIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IDTClass: *mut DTClassIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn dtclass_obtain_engine() -> *mut APTR {
    ((*IDTClass).ObtainEngine)(IDTClass)
}
