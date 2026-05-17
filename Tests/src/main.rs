//! Test-suite orchestrator for rust-for-amigaos4.
//!
//! Runs every layer of the test pyramid and prints a single aggregate
//! summary. Layers:
//!   1. Tests/ own integration tests (build/repro, doc claims, regression)
//!   2. In-source `#[cfg(test)] mod tests {}` blocks in each library crate
//!      (driven by `cargo test` per crate)
//!   3. Doctests in each library crate
//!
//! Target-side cross-compile tests (`examples/test-harness*`) and QEMU
//! smoke tests are out of scope for this orchestrator.

use amiga_tests::{cargo_test_in, is_windows_host, repo_root, strip_ansi};
use std::process::ExitCode;

struct StepResult {
    name: &'static str,
    status: StepStatus,
    detail: String,
}

enum StepStatus {
    Passed,
    Failed,
    Skipped,
}

impl StepStatus {
    fn glyph(&self) -> &'static str {
        match self {
            StepStatus::Passed => "PASS",
            StepStatus::Failed => "FAIL",
            StepStatus::Skipped => "SKIP",
        }
    }
}

fn main() -> ExitCode {
    println!("rust-for-amigaos4 test suite — orchestrator");
    println!("repo root: {}", repo_root().display());
    println!();

    let mut results: Vec<StepResult> = Vec::new();

    // ── Layer 1: Tests/ own integration tests ────────────────────
    results.push(run_self_tests());

    // ── Layer 2 + 3: in-source unit tests + doctests per crate ───
    for crate_dir in ["amigaos4-sys", "amigaos4-alloc", "amigaos4"] {
        results.push(run_in_crate_tests(crate_dir));
    }

    println!();
    println!("════════════════════════════════════════════");
    println!("  Summary");
    println!("════════════════════════════════════════════");

    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    for r in &results {
        println!("  [{}] {}  — {}", r.status.glyph(), r.name, r.detail);
        match r.status {
            StepStatus::Passed => passed += 1,
            StepStatus::Failed => failed += 1,
            StepStatus::Skipped => skipped += 1,
        }
    }
    println!();
    println!("  total: {} passed, {} failed, {} skipped",
             passed, failed, skipped);

    if failed > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

fn run_self_tests() -> StepResult {
    println!("── Tests/ integration tests ──");
    let out = std::process::Command::new("cargo")
        .args(["test", "--no-fail-fast"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to spawn cargo test in Tests/");
    let stdout = strip_ansi(&String::from_utf8_lossy(&out.stdout));
    let stderr = strip_ansi(&String::from_utf8_lossy(&out.stderr));
    print!("{}", stdout);
    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr);
    }
    let detail = summarize_cargo_output(&stdout);
    StepResult {
        name: "Tests/ self",
        status: if out.status.success() { StepStatus::Passed } else { StepStatus::Failed },
        detail,
    }
}

fn run_in_crate_tests(crate_dir: &'static str) -> StepResult {
    println!("── in-crate tests: {} ──", crate_dir);
    // Known Windows linker issue with amigaos4 — see Tests/README.md.
    if is_windows_host() && crate_dir == "amigaos4" {
        let detail = "skipped on Windows host (unresolved IExec/IDOS/IIntuition \
                      globals — Linux CI links these lazily)".to_string();
        println!("  SKIP: {}", detail);
        return StepResult { name: crate_name(crate_dir), status: StepStatus::Skipped, detail };
    }

    let out = cargo_test_in(crate_dir);
    let stdout = strip_ansi(&String::from_utf8_lossy(&out.stdout));
    let stderr = strip_ansi(&String::from_utf8_lossy(&out.stderr));
    print!("{}", stdout);
    if !stderr.trim().is_empty() {
        eprintln!("{}", stderr);
    }
    let detail = summarize_cargo_output(&stdout);
    StepResult {
        name: crate_name(crate_dir),
        status: if out.status.success() { StepStatus::Passed } else { StepStatus::Failed },
        detail,
    }
}

fn crate_name(crate_dir: &str) -> &'static str {
    match crate_dir {
        "amigaos4-sys" => "amigaos4-sys cargo test",
        "amigaos4-alloc" => "amigaos4-alloc cargo test",
        "amigaos4" => "amigaos4 cargo test",
        _ => "unknown crate",
    }
}

/// Aggregate counts across every `test result: ...` line cargo emits
/// (one per test binary, plus one for doctests). Cargo's format is:
/// `test result: ok. N passed; N failed; N ignored; ...`
fn summarize_cargo_output(stdout: &str) -> String {
    let mut total_passed = 0u64;
    let mut total_failed = 0u64;
    let mut found_any = false;
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("test result: ") {
            found_any = true;
            for token in rest.split(';') {
                let t = token.trim();
                // Each token is like "ok. 5 passed", " 0 failed", etc.
                // Extract the FIRST run of digits, then identify the label.
                let num_str: String = t.chars()
                    .skip_while(|c| !c.is_ascii_digit())
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                if num_str.is_empty() { continue; }
                let n: u64 = num_str.parse().unwrap_or(0);
                if t.ends_with(" passed") { total_passed += n; }
                else if t.ends_with(" failed") { total_failed += n; }
            }
        }
    }
    if !found_any {
        "no 'test result' line in cargo output".to_string()
    } else {
        format!("{} passed, {} failed (aggregated across binaries)",
                total_passed, total_failed)
    }
}
