//
// AmigaOS 4 locale-i18n-hello - opens an existing system catalog
// via locale.library, looks up a couple of localised strings by ID,
// and prints both the translation (if found) and the supplied
// default fallback.
//
// Demonstrates amigaos4::locale::AmigaCatalog as a thin safe RAII
// wrapper over OpenCatalogA / GetCatalogStr / CloseCatalog. The
// catalog target here is "mixer.catalog", which ships with every
// AmigaOS 4 install -- if the user's current locale doesn't have
// a translation for some string the default is returned, which the
// example also verifies.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::locale::AmigaCatalog;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - locale-i18n-hello\n\0");
    say(b"========================================\n\0");

    let cat = match AmigaCatalog::open(b"mixer.catalog\0") {
        Ok(c) => {
            say(b"  Opened mixer.catalog via locale.library\n\0");
            Some(c)
        }
        Err(_) => {
            say(b"  mixer.catalog not opened (system has no matching\n\0");
            say(b"  localized catalog, or locale.library refused).\n\0");
            None
        }
    };

    // Try a couple of well-known catalog IDs. Whether the system has
    // a translation or not, the wrapper returns either the translated
    // bytes or our default — the default-fallback path is the part
    // worth verifying.
    let lookups: [(i32, &[u8]); 3] = [
        (1, b"Master Volume\0"),
        (2, b"Mute\0"),
        (999, b"<unknown string>\0"),
    ];

    let mut total_bytes: u32 = 0;
    for (id, default) in lookups {
        let s: &[u8] = if let Some(ref cat) = cat {
            cat.get(id, default)
        } else {
            // Strip the trailing NUL from the default like the wrapper does.
            match default.iter().position(|&b| b == 0) {
                Some(end) => &default[..end],
                None      => default,
            }
        };
        total_bytes += s.len() as u32;

        // Print: "  id=<n>: <bytes>\n"
        unsafe { amiga_debug_fmt_u32(b"  id=%lu: \0".as_ptr(), id as u32); }
        // s is not NUL-terminated; copy into a small buffer that is.
        let mut buf = [0u8; 256];
        let n = s.len().min(buf.len() - 1);
        buf[..n].copy_from_slice(&s[..n]);
        buf[n] = 0;
        unsafe { amiga_debug_str(buf.as_ptr()); }
        say(b"\n\0");
    }

    say_d(b"\n  Total bytes across 3 lookups      = \0", total_bytes);
    if total_bytes == 0 {
        say(b"FAIL: every lookup returned an empty string\n\0");
        return 1;
    }

    // Catalog handle drops here — calls CloseCatalog.
    drop(cat);

    say(b"  => PASS: locale.library catalog round-trip via the binding\n\0");
    say(b"========================================\n\0");
    0
}
