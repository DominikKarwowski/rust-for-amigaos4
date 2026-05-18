//! ITextEditor global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `TEXTEDITOR_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::texteditor::*;

// ---- ITextEditor (TextEditorIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ITextEditor: *mut TextEditorIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ITextEditor: *mut TextEditorIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn texteditor_texteditor_get_class() -> *mut APTR {
    ((*ITextEditor).TEXTEDITOR_GetClass)(ITextEditor)
}
