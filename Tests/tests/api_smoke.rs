//! Public-API smoke tests (host-runnable).
//!
//! These don't import the production crates as path deps (that would hit
//! the Windows-linker issue documented in Tests/README.md). Instead they:
//!
//!   1. Parse the crate source files to verify the public-API surface
//!      matches what the docs claim.
//!   2. Drive `cargo check` with various feature combinations to verify
//!      the feature matrix compiles (typechecks, no linking).
//!
//! Together this gives the "downstream consumer" angle without needing
//! amigaos4 to link on the host.

use amiga_tests::{read_repo_file, repo_root};
use std::process::Command;

// ── Module surface verification ──────────────────────────────

#[test]
fn amigaos4_lib_rs_declares_expected_modules() {
    // From CLAUDE.md: 25 modules total.
    let core_modules: &[&str] = &[
        "error", "tag", "mem", "port", "screen", "boopsi", "window", "gfx",
        "requester", "reaction", "dos", "locale", "io", "fmt", "panic",
        "async_rt", "timer", "clipboard",
    ];
    let app_modules: &[&str] = &[
        "fs", "time", "env", "thread", "net", "dns", "http",
    ];

    let lib_src = read_repo_file("amigaos4/src/lib.rs");
    let mut missing: Vec<&str> = Vec::new();

    for m in core_modules.iter().chain(app_modules.iter()) {
        // Module is declared as `pub mod X;` somewhere.
        let needle = format!("pub mod {};", m);
        if !lib_src.contains(&needle) {
            missing.push(*m);
        }
    }

    assert!(missing.is_empty(),
        "amigaos4/src/lib.rs is missing module declarations for: {:?}",
        missing);

    let actual = core_modules.len() + app_modules.len();
    assert_eq!(actual, 25,
        "core+app module count is {}, docs say 25 — update either", actual);
}

#[test]
fn amigaos4_module_files_exist_for_each_declared_module() {
    let modules: &[&str] = &[
        "error", "tag", "mem", "port", "screen", "boopsi", "window", "gfx",
        "requester", "reaction", "dos", "locale", "io", "fmt", "panic",
        "async_rt", "timer", "clipboard",
        "fs", "time", "env", "thread", "net", "dns", "http",
    ];
    let mut missing: Vec<&str> = Vec::new();
    for m in modules {
        let p = repo_root().join("amigaos4").join("src").join(format!("{}.rs", m));
        if !p.exists() {
            missing.push(*m);
        }
    }
    assert!(missing.is_empty(),
        "modules declared but source files missing: {:?}", missing);
}

// ── Feature gates ───────────────────────────────────────────

#[test]
fn amigaos4_sys_feature_count_matches_docs() {
    // CLAUDE.md / README say "129 feature-gated interfaces".
    let toml = read_repo_file("amigaos4-sys/Cargo.toml");
    // Count single-feature lines like `foo = []`. Exclude `default`,
    // `full-sdk`, and any aggregate.
    let mut features: Vec<&str> = Vec::new();
    let mut in_features = false;
    for line in toml.lines() {
        let l = line.trim();
        if l == "[features]" { in_features = true; continue; }
        if l.starts_with('[') && l != "[features]" {
            in_features = false;
        }
        if !in_features { continue; }
        if let Some(eq) = l.find('=') {
            let name = l[..eq].trim();
            let rhs = l[eq + 1..].trim();
            // Single-interface feature has empty list rhs; aggregates have items.
            if rhs == "[]" && !name.is_empty() && !name.starts_with('#') {
                features.push(name);
            }
        }
    }

    assert_eq!(features.len(), 129,
        "amigaos4-sys exposes {} single-interface features, docs say 129", features.len());
}

#[test]
fn amigaos4_default_features_are_app_only() {
    let toml = read_repo_file("amigaos4/Cargo.toml");
    assert!(toml.contains(r#"default = ["app"]"#),
        "amigaos4 default features changed — update docs if intentional");
}

// ── cargo check with various features ───────────────────────

#[test]
fn amigaos4_sys_default_features_check() {
    let out = Command::new("cargo")
        .args(["check", "--manifest-path", "amigaos4-sys/Cargo.toml"])
        .current_dir(repo_root())
        .output()
        .expect("cargo check failed to spawn");
    assert!(out.status.success(),
        "cargo check on amigaos4-sys (default features) failed:\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr));
}

#[test]
fn amigaos4_sys_no_default_features_check() {
    let out = Command::new("cargo")
        .args(["check", "--manifest-path", "amigaos4-sys/Cargo.toml",
               "--no-default-features"])
        .current_dir(repo_root())
        .output()
        .expect("cargo check failed to spawn");
    assert!(out.status.success(),
        "cargo check on amigaos4-sys (no default features) failed:\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr));
}

#[test]
fn amigaos4_alloc_exec_feature_check() {
    let out = Command::new("cargo")
        .args(["check", "--manifest-path", "amigaos4-alloc/Cargo.toml",
               "--features", "exec"])
        .current_dir(repo_root())
        .output()
        .expect("cargo check failed to spawn");
    assert!(out.status.success(),
        "cargo check on amigaos4-alloc --features exec failed:\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr));
}

#[test]
fn amigaos4_driver_features_check() {
    // driver feature set excludes the POSIX modules.
    let out = Command::new("cargo")
        .args(["check", "--manifest-path", "amigaos4/Cargo.toml",
               "--no-default-features", "--features", "driver"])
        .current_dir(repo_root())
        .output()
        .expect("cargo check failed to spawn");
    assert!(out.status.success(),
        "cargo check on amigaos4 --features driver failed:\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr));
}
