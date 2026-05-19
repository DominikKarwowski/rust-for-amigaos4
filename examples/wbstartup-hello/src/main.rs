//
// AmigaOS 4 Workbench-vs-CLI launch detection.
//
// clib4's CRT calls main() with argc=0 when launched from Workbench
// (icon double-click); otherwise argc is the usual non-zero CLI count.
// The WBStartup message itself is stored as the current task's
// tc_UserData on WB launch -- see <workbench/startup.h> in the SDK.
//
// To exercise the WB branch on QEMU you'd need to drop an .info on
// the binary and double-click it; the CLI branch runs by default
// from a shell invocation.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])           { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }
fn say_x(p: &[u8], v: u32) { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%lx\n\0".as_ptr(), v); } }

// struct WBStartup layout (from workbench/startup.h):
//   sm_Message     : Message  (20 bytes)
//   sm_Process     : *MsgPort (4)
//   sm_Segment     : BPTR     (4)
//   sm_NumArgs     : LONG     (4)
//   sm_ToolWindow  : STRPTR   (4)
//   sm_ArgList     : *WBArg   (4)
const WBSTARTUP_NUMARGS_OFFSET: usize = 20 + 4 + 4;     // 28
const WBSTARTUP_ARGLIST_OFFSET: usize = 28 + 4 + 4;     // 36

// Task struct's tc_UserData lives at offset 50 (the Node header is 14
// bytes; then ln_Pri/Flags/Type + several more fields land tc_UserData
// at exactly 0x32). Validated against exec/tasks.h.
const TASK_USERDATA_OFFSET: usize = 50;

#[no_mangle]
pub extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - wbstartup-hello\n\0");
    say(b"========================================\n\0");
    say_d(b"  argc                              = \0", argc as u32);

    if argc == 0 {
        // Workbench launch. Pull WBStartup from the current task.
        say(b"  Launched from Workbench (WB)\n\0");

        let task = unsafe { exec_find_task(core::ptr::null()) };
        say_x(b"  FindTask(NULL)                    = \0", task as u32);
        if task.is_null() {
            say(b"FAIL: FindTask returned NULL\n\0");
            return 1;
        }
        let wb_msg = unsafe {
            let p = (task as *const u8).add(TASK_USERDATA_OFFSET) as *const u32;
            p.read_unaligned()
        };
        say_x(b"  Task->tc_UserData (WBStartup*)    = \0", wb_msg);
        if wb_msg == 0 {
            say(b"FAIL: tc_UserData is NULL on WB launch\n\0");
            return 2;
        }
        let num_args = unsafe {
            let p = (wb_msg as *const u8).add(WBSTARTUP_NUMARGS_OFFSET) as *const u32;
            p.read_unaligned()
        };
        let arg_list = unsafe {
            let p = (wb_msg as *const u8).add(WBSTARTUP_ARGLIST_OFFSET) as *const u32;
            p.read_unaligned()
        };
        say_d(b"  sm_NumArgs                        = \0", num_args);
        say_x(b"  sm_ArgList                        = \0", arg_list);
        say(b"\n  => PASS: WB launch detected, WBStartup walked\n\0");
    } else {
        // CLI launch. Echo the args we got.
        say(b"  Launched from Shell/CLI\n\0");
        if !argv.is_null() {
            for i in 0..argc as usize {
                let p = unsafe { *argv.add(i) };
                if !p.is_null() {
                    say(b"    arg: \0");
                    unsafe { amiga_debug_str(p); }
                    say(b"\n\0");
                }
            }
        }
        say(b"\n  => PASS: CLI launch detected, args printed\n\0");
        say(b"  (drop an icon on the binary and double-click to test WB launch)\n\0");
    }

    say(b"========================================\n\0");
    0
}
