# rust-examples
Rust Examples for Introduction Course

## Windows Instructions
Handy commands for Windows users:

```powershell
# One-line installation of probe-rs
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-installer.ps1 | iex

# Alternatively, you can install probe-rs manually:

# Prerequisites for building probe-rs
# bootstrap vcpkg
.\vcpkg\bootstrap-vcpkg.bat

# dynamic linking 64-bit
.\vcpkg\vcpkg install libftdi1:x64-windows libusb:x64-windows
set VCPKGRS_DYNAMIC=1

# static linking 64-bit
.\vcpkg\vcpkg install libftdi1:x64-windows-static-md libusb:x64-windows-static-md
```

## Unix Instructions
Handy commands for Linux users:

```bash
# Prerequisites for building probe-rs

# Debian/Ubuntu
sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev libssl-dev

# Fedora / RedHat / CentOS
sudo dnf install libusbx-devel libftdi-devel libudev-devel openssl-devel

# macOS
brew install libftdi
```
