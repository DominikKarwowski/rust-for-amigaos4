# Tests — black-box test suite for rust-for-amigaos4

A standalone Rust crate that verifies project-level invariants from
outside the three production crates. Sits alongside the in-crate unit
tests and doctests, and orchestrates them as a single combined run.

## The three sections

Tests in this suite fall into one of three categories. The category is
encoded in the test file's filename prefix.

### 1. `rust_*` — Rust-only

Tests that need a host development environment but never call into the
AmigaOS runtime. They verify project structure, build infrastructure,
documentation, and the FFI shape. Always host-runnable.

| File | What it checks |
|---|---|
| `tests/rust_repro.rs` | Cargo targets resolve; example/template Makefiles parse; clib4-nightly binaries are present and reasonable; submodule pin matches docs. |
| `tests/rust_api_smoke.rs` | Public API surface — module declarations, source files, feature flags, `cargo check` matrix. |
| `tests/rust_doc_claims.rs` | Counts and assertions in README/CLAUDE.md/roadmap match reality. |
| `tests/rust_regression.rs` | Specific past bugs cannot return silently. |
| `tests/rust_signature_audit.rs` | Every regular vtable method in the 19 wrappered interfaces has a Rust wrapper with the bindgen-canonical snake_case name; every feature flag has an `interfaces/<name>.rs`. **Fails on any gap** — see gap policy below. |
| `tests/rust_sdk_audit.rs` | Every method declared in the AmigaOS 4 SDK interface headers is present in the Rust binding. Runs opportunistically: if the SDK is found at `C:\msys64\home\rich_\sdk` (or `$AMIGA_SDK`), the audit executes and **fails on any gap**; otherwise the test passes with a notice on stderr. |

### 2. `combined_*` — Combined

Tests of pure-Rust logic that is identical on host and PPC: error
formatting, `Duration` arithmetic, `TagListBuilder` construction, the
`Read`/`Write` trait contract. Compile-tested on host via `cargo test`;
the same source could be compiled into a PPC binary and re-run on
AmigaOS to verify no surprises in the cross-build.

| File | What it checks |
|---|---|
| `tests/combined_amigaos4.rs` | The pure-Rust integration tests migrated from `amigaos4/tests/host_tests.rs`. Gated behind the `link-amigaos4` feature (on by default; pass `--no-default-features` to skip). |

### 3. AmigaOS4-only — target-side test harnesses

Tests that require a live AmigaOS runtime (`IExec`, file system,
intuition, etc.). Today these live as cross-compile projects under
`examples/test-harness*/`. They build with `./build.sh` and run on
AmigaOS 4 (QEMU or real hardware). The Tests/ orchestrator does **not**
build or run them — that requires Docker + QEMU. Driving them is a
future addition; for now they're invoked manually.

| Example | What it exercises |
|---|---|
| `examples/test-harness` | Core OS-call tests: `Vec`, `String`, `TagList`, `AmigaVec`, `MsgPort`, `Duration`, file I/O, env, timer, clipboard. |
| `examples/test-harness-gui` | ReAction window/gadget tests on the actual Intuition framework. |
| `examples/test-harness-net` | Real TCP/DNS/HTTP against the AmigaOS networking stack. |

## SDK coverage policy

> All AmigaOS 4 SDK methods/functions implemented in Rust must be
> covered by the test suite.

Two audits enforce this together:

`rust_sdk_audit.rs` is the authoritative outer check — it compares
each Rust IFace struct against the actual SDK header (`exec.h`,
`dos.h`, etc.) and reports any SDK method that isn't bound in Rust.
This runs on machines where the AmigaOS 4 SDK is installed (default:
`C:\msys64\home\rich_\sdk`); skips with a notice otherwise.

`rust_signature_audit.rs` is the inner consistency check:

* For every regular method in each of the 19 vtable structs in
  `amigaos4-sys/src/interfaces/<name>.rs`, the audit requires a
  matching wrapper in `amigaos4-sys/src/wrappers/<name>.rs` with the
  bindgen-canonical snake_case name.
* `Obtain` / `Release` and `*_UNIMPLEMENTED` slots are skipped — they
  are framework-managed and have no Rust wrappers.
* Varargs slots (typed as `APTR`) are skipped — they are wrapped via C
  glue in `amigaos4-sys/glue/amiga_glue.c`, not Rust.
* For every feature flag in `amigaos4-sys/Cargo.toml`, the audit
  requires a matching `interfaces/<name>.rs`. Catches "feature added
  but bindings forgotten" and the reverse.

**Gap policy: fail the build.** Any missing wrapper or orphaned
interface file fails the test, surfacing the gap on the next CI run.

## Running

```bash
# Everything host-runnable (Rust-only + combined + per-crate in-source tests)
cargo run --bin amiga-test-runner

# Just the Tests/ own integration tests
cargo test

# Excluding the migrated amigaos4 host tests (if amigaos4 doesn't link)
cargo test --no-default-features

# A single category
cargo test --test rust_signature_audit
```

## In-source tests stay put

`#[cfg(test)] mod tests {}` blocks inside each library's source files
and doctests in `///` doc comments remain where they are — they
exercise private internals and are inherently tied to their source.
The orchestrator picks them up by spawning `cargo test` per crate.
