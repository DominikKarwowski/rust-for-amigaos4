//! Build & reproducibility tests.
//!
//! Verifies that the build infrastructure is internally consistent:
//!   - every example/template has the file layout templates declare
//!   - cargo configs enable the unstable features the target spec needs
//!   - the clib4-nightly overlay binaries are present and non-empty
//!   - the clib4 submodule HEAD matches what the docs assert
//!
//! Heavy cross-compile and Docker rebuild checks are deliberately out of
//! scope here (they belong in CI, not the host-runnable suite).

use amiga_tests::{read_repo_file, repo_root};
use std::fs;

const EXAMPLES: &[&str] = &[
    "hello",
    "hello-driver",
    "hello-library",
    "test-harness",
    "test-harness-gui",
    "test-harness-net",
    "file-io-demo",
    "timer-demo",
    "thread-demo",
    "gui-demo",
    "net-demo",
    "async-demo",
];

const TEMPLATES: &[&str] = &["app", "driver", "library"];

const CRATES: &[&str] = &["amigaos4", "amigaos4-sys", "amigaos4-alloc"];

// ── Per-example file layout ──────────────────────────────────

#[test]
fn every_example_has_required_files() {
    let root = repo_root();
    let mut missing: Vec<String> = Vec::new();
    for ex in EXAMPLES {
        for required in &[
            "Cargo.toml",
            "Makefile",
            "powerpc-amigaos.json",
            ".cargo/config.toml",
            "fake-linker.bat",
            "fake-linker.sh",
        ] {
            let p = root.join("examples").join(ex).join(required);
            if !p.exists() {
                missing.push(format!("examples/{}/{}", ex, required));
            }
        }
        // Every example has either src/main.rs or src/lib.rs.
        let src_main = root.join("examples").join(ex).join("src").join("main.rs");
        let src_lib = root.join("examples").join(ex).join("src").join("lib.rs");
        if !src_main.exists() && !src_lib.exists() {
            missing.push(format!("examples/{}/src/{{main,lib}}.rs", ex));
        }
    }
    assert!(missing.is_empty(), "missing files:\n  - {}", missing.join("\n  - "));
}

#[test]
fn every_template_has_required_files() {
    let root = repo_root();
    let mut missing: Vec<String> = Vec::new();
    for tpl in TEMPLATES {
        for required in &[
            "Cargo.toml",
            "Makefile",
            "powerpc-amigaos.json",
            ".cargo/config.toml",
            "src",
        ] {
            let p = root.join("templates").join(tpl).join(required);
            if !p.exists() {
                missing.push(format!("templates/{}/{}", tpl, required));
            }
        }
    }
    assert!(missing.is_empty(), "missing files:\n  - {}", missing.join("\n  - "));
}

// ── Example Cargo.toml shape ────────────────────────────────

#[test]
fn every_example_cargo_toml_has_staticlib_crate_type() {
    let mut bad: Vec<String> = Vec::new();
    for ex in EXAMPLES {
        let text = read_repo_file(format!("examples/{}/Cargo.toml", ex));
        if !text.contains("crate-type") || !text.contains("staticlib") {
            bad.push(format!("examples/{}", ex));
        }
    }
    assert!(bad.is_empty(),
        "examples missing staticlib crate-type:\n  - {}",
        bad.join("\n  - "));
}

#[test]
fn every_example_panics_abort() {
    let mut bad: Vec<String> = Vec::new();
    for ex in EXAMPLES {
        let text = read_repo_file(format!("examples/{}/Cargo.toml", ex));
        if !text.contains(r#"panic = "abort""#) {
            bad.push(format!("examples/{}", ex));
        }
    }
    assert!(bad.is_empty(),
        "examples without panic=abort:\n  - {}",
        bad.join("\n  - "));
}

// ── .cargo/config.toml unstable feature flags ───────────────

#[test]
fn every_example_enables_required_unstable_flags() {
    let required = [
        "build-std",
        "json-target-spec",
    ];
    let mut bad: Vec<(String, &'static str)> = Vec::new();
    for ex in EXAMPLES {
        let text = read_repo_file(format!("examples/{}/.cargo/config.toml", ex));
        for flag in &required {
            if !text.contains(flag) {
                bad.push((format!("examples/{}", ex), flag));
            }
        }
    }
    assert!(bad.is_empty(),
        "examples missing unstable flags:\n  - {}",
        bad.iter().map(|(p, f)| format!("{} (needs {})", p, f))
                  .collect::<Vec<_>>().join("\n  - "));
}

// ── Target spec ─────────────────────────────────────────────

#[test]
fn every_example_target_spec_is_powerpc() {
    let mut bad: Vec<String> = Vec::new();
    for ex in EXAMPLES {
        let text = read_repo_file(format!("examples/{}/powerpc-amigaos.json", ex));
        // Must declare PPC arch and no_std-friendly settings.
        if !text.contains(r#""arch""#) || !text.contains(r#""powerpc""#) {
            bad.push(format!("examples/{}", ex));
        }
    }
    assert!(bad.is_empty(),
        "examples with non-PPC target spec:\n  - {}",
        bad.join("\n  - "));
}

// ── clib4-nightly overlay ───────────────────────────────────

#[test]
fn clib4_nightly_library_files_exist_and_are_nonempty() {
    let root = repo_root();
    for name in &["clib4.library", "clib4.library.debug"] {
        let p = root.join("clib4-nightly").join(name);
        assert!(p.exists(), "missing {}", p.display());
        let sz = fs::metadata(&p).unwrap().len();
        assert!(sz > 100_000,
            "{} suspiciously small ({} bytes) — corrupt overlay?",
            p.display(), sz);
    }
}

#[test]
fn clib4_nightly_lib_has_clib4_and_crt_objects() {
    let lib = repo_root().join("clib4-nightly").join("lib");
    for required in &[
        "libc.a", "libm.a", "libpthread.a", "libamiga.a",
        "libcrypt.a", "libdebug.a", "libprofile.a", "librt.a",
        "crt0.o", "crtbegin.o", "crtend.o",
    ] {
        let p = lib.join(required);
        assert!(p.exists(), "missing clib4-nightly/lib/{}", required);
        assert!(fs::metadata(&p).unwrap().len() > 0,
            "clib4-nightly/lib/{} is zero bytes", required);
    }
}

#[test]
fn clib4_nightly_include_has_pthread_h() {
    let p = repo_root().join("clib4-nightly").join("include").join("pthread.h");
    assert!(p.exists(), "clib4-nightly/include/pthread.h missing");
    let text = fs::read_to_string(&p).unwrap();
    // Sentinel from the post-bump fix
    assert!(text.contains("pthread_once_t"),
        "pthread.h appears truncated — pthread_once_t typedef missing");
}

// ── Submodule pin matches what docs say ─────────────────────

#[test]
fn clib4_submodule_pinned_in_gitmodules() {
    let text = read_repo_file(".gitmodules");
    assert!(text.contains(r#"[submodule "clib4-src"]"#),
        ".gitmodules missing clib4-src entry");
    assert!(text.contains("AmigaLabs/clib4"),
        ".gitmodules clib4-src url is not AmigaLabs/clib4");
}

#[test]
fn clib4_submodule_head_matches_docs() {
    // Pull the SHA the parent project has recorded for clib4-src/
    let out = std::process::Command::new("git")
        .args(["ls-tree", "HEAD", "clib4-src"])
        .current_dir(repo_root())
        .output()
        .expect("git ls-tree failed");
    let line = String::from_utf8_lossy(&out.stdout);
    // Format: "160000 commit <sha>\tclib4-src"
    let sha = line.split_whitespace().nth(2)
        .unwrap_or("")
        .to_string();
    assert!(!sha.is_empty(),
        "could not find clib4-src SHA in git ls-tree output: {}", line);

    let short = &sha[..8];
    // The docs MUST mention the current short SHA in at least one place.
    let docs_to_check = ["README.md", "CLAUDE.md", "docs/roadmap.md"];
    let mut found_any = false;
    for d in &docs_to_check {
        let text = read_repo_file(d);
        if text.contains(short) {
            found_any = true;
            break;
        }
    }
    assert!(found_any,
        "docs do not reference current clib4 pin {} ({}). Update README.md / CLAUDE.md / docs/roadmap.md.",
        short, sha);
}

// ── Crate Cargo.toml sanity ──────────────────────────────────

#[test]
fn every_crate_cargo_toml_has_edition_2021() {
    let mut bad: Vec<String> = Vec::new();
    for c in CRATES {
        let text = read_repo_file(format!("{}/Cargo.toml", c));
        if !text.contains(r#"edition = "2021""#) {
            bad.push(c.to_string());
        }
    }
    assert!(bad.is_empty(), "crates not on edition 2021: {:?}", bad);
}

