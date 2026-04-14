@echo off
REM setup.bat — One-time setup for Rust-for-AmigaOS4 (Windows)
REM
REM Installs Rust nightly, rust-src component, and pulls the Docker image.
REM Run once before your first build.

echo === Rust for AmigaOS 4 — Setup ===
echo.

REM 1. Check rustup
where rustup >nul 2>&1
if errorlevel 1 (
    echo ERROR: rustup not found. Install from https://rustup.rs/
    exit /b 1
)

REM 2. Initialise submodules (clib4 source pinned in clib4-src\)
echo [1/4] Initialising git submodules (clib4 source)...
git submodule update --init --recursive
echo   Done.

REM 3. Install Rust toolchain pinned by rust-toolchain.toml
echo [2/4] Installing Rust toolchain pinned by rust-toolchain.toml...
rustup show active-toolchain >nul
echo   Done.

REM 4. Install rust-src (needed for build-std)
echo [3/4] Ensuring rust-src component is present...
rustup component add rust-src
echo   Done.

REM 5. Pull Docker image (via WSL)
echo [4/4] Pulling AmigaOS cross-compiler Docker image via WSL...
wsl sh -c "docker pull walkero/amigagccondocker:os4-gcc11"
echo   Done.

echo.
echo === Setup complete ===
echo.
echo Next steps:
echo   cd examples\hello
echo   build.bat
