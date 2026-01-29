# Helix UEFI Boot Platform

## 🚀 Revolutionary UEFI Boot Implementation in Rust

This is not just another UEFI bootloader. This is a **complete UEFI boot platform**
designed to become the reference implementation for modern, secure, Rust-based
operating system bootstrapping.

## Vision

- **Reinvent UEFI boot** with Rust's safety guarantees, strong typing, and auditability
- **Create an elegant abstraction layer** between UEFI firmware and kernel
- **Establish a new standard** for Rust-based boot infrastructure

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    KERNEL (Helix Core)                       │
├─────────────────────────────────────────────────────────────┤
│                     Boot Handoff Layer                       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────────┐   │
│  │BootInfo │ │MemMap  │ │Graphics │ │ Hardware Tables │   │
│  └─────────┘ └─────────┘ └─────────┘ └─────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                  UEFI Protocol Abstractions                  │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐  │
│  │GOP │ │Text│ │FS  │ │Blk │ │PCI │ │USB │ │Net │ │Ser │  │
│  └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘ └────┘  │
├─────────────────────────────────────────────────────────────┤
│                     Security Layer                           │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │SecureBoot│ │Measured  │ │Signature │ │ Trust Chain  │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                    Raw UEFI Bindings                         │
│  ┌───────────┐ ┌─────────────┐ ┌─────────────────────────┐ │
│  │SystemTable│ │BootServices │ │    RuntimeServices      │ │
│  └───────────┘ └─────────────┘ └─────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                     UEFI Firmware                            │
└─────────────────────────────────────────────────────────────┘
```

## Features

### Core UEFI Support
- ✅ Complete System Table handling
- ✅ Boot Services wrapper (memory, protocols, events)
- ✅ Runtime Services wrapper (time, variables, reset)
- ✅ Safe ExitBootServices transition

### Protocols
- ✅ **GOP** - Graphics Output Protocol with multi-resolution support
- ✅ **Simple Text** - Console input/output
- ✅ **File System** - FAT/ESP filesystem access
- ✅ **Block I/O** - Raw disk access
- ✅ **PCI** - PCI device enumeration
- ✅ **Serial I/O** - Debug output
- ✅ **RNG** - Hardware random number generator

### Hardware Tables
- ✅ **ACPI** - Full ACPI table parsing (RSDP, XSDT, MADT, FADT, etc.)
- ✅ **SMBIOS** - System information parsing
- ✅ **Device Tree** - DTB parsing for ARM platforms

### Security
- ✅ **Secure Boot** - Signature validation chain
- ✅ **Measured Boot** - TPM PCR measurements
- ✅ **Trust Chain** - Verified boot path

### Advanced Features
- ✅ **SMP** - Multi-processor initialization
- ✅ **NUMA** - NUMA topology detection
- ✅ **Memory Model** - Safe Rust memory map abstraction

## Module Organization

```
helix-uefi/
├── src/
│   ├── lib.rs              # Main library entry
│   ├── bin/
│   │   └── main.rs         # UEFI application entry point
│   │
│   ├── raw/                # Layer 0: Raw UEFI bindings
│   │   ├── mod.rs
│   │   ├── types.rs        # UEFI base types (GUID, Handle, Status)
│   │   ├── system_table.rs # EFI_SYSTEM_TABLE
│   │   ├── boot_services.rs
│   │   ├── runtime_services.rs
│   │   └── protocols/      # Raw protocol definitions
│   │
│   ├── services/           # Layer 1: Safe service wrappers
│   │   ├── mod.rs
│   │   ├── boot.rs         # Safe Boot Services
│   │   ├── runtime.rs      # Safe Runtime Services
│   │   ├── memory.rs       # Memory allocation
│   │   ├── protocol.rs     # Protocol handling
│   │   └── event.rs        # Event/Timer handling
│   │
│   ├── protocols/          # Layer 2: Protocol abstractions
│   │   ├── mod.rs
│   │   ├── console/        # Text I/O
│   │   ├── graphics/       # GOP
│   │   ├── filesystem/     # File System
│   │   ├── block/          # Block I/O
│   │   ├── pci/            # PCI
│   │   ├── serial/         # Serial I/O
│   │   └── network/        # Network
│   │
│   ├── tables/             # Layer 3: Hardware tables
│   │   ├── mod.rs
│   │   ├── acpi/           # ACPI parsing
│   │   ├── smbios/         # SMBIOS parsing
│   │   └── dtb/            # Device Tree
│   │
│   ├── security/           # Layer 4: Security
│   │   ├── mod.rs
│   │   ├── secure_boot.rs
│   │   ├── measured_boot.rs
│   │   ├── signature.rs
│   │   └── trust_chain.rs
│   │
│   ├── memory/             # Memory management
│   │   ├── mod.rs
│   │   ├── map.rs          # Memory map
│   │   ├── allocator.rs    # UEFI allocator
│   │   ├── physical.rs     # Physical addresses
│   │   └── virtual.rs      # Virtual addresses
│   │
│   ├── loader/             # Kernel loading
│   │   ├── mod.rs
│   │   ├── elf.rs          # ELF loader
│   │   ├── pe.rs           # PE loader
│   │   └── kernel.rs       # Kernel loading logic
│   │
│   ├── handoff/            # Kernel handoff
│   │   ├── mod.rs
│   │   ├── boot_info.rs    # Boot information structure
│   │   ├── exit.rs         # ExitBootServices handling
│   │   └── transition.rs   # Firmware → Kernel transition
│   │
│   └── arch/               # Architecture-specific
│       ├── mod.rs
│       ├── x86_64/
│       └── aarch64/
```

## Usage

### As a UEFI Application

```rust
#![no_std]
#![no_main]

use helix_uefi::prelude::*;

#[entry]
fn efi_main(image: Handle, system_table: &SystemTable) -> Status {
    // Initialize the UEFI environment
    let mut env = UefiEnvironment::init(image, system_table)?;

    // Print to console
    env.console().println("Helix UEFI Boot Platform");

    // Get memory map
    let memory_map = env.boot_services().memory_map()?;

    // Load kernel
    let kernel = env.load_kernel("\\EFI\\HELIX\\KERNEL.ELF")?;

    // Prepare handoff
    let boot_info = BootInfoBuilder::new()
        .with_memory_map(memory_map)
        .with_framebuffer(env.graphics()?)
        .with_acpi(env.acpi_tables()?)
        .build();

    // Exit boot services and jump to kernel
    env.exit_and_jump(kernel, boot_info)
}
```

### As a Library

```rust
use helix_uefi::{
    SystemTable, BootServices, RuntimeServices,
    protocols::{GraphicsOutput, FileSystem},
    tables::{Acpi, Smbios},
    handoff::BootInfo,
};
```

## Building

```bash
# Build for x86_64 UEFI
cargo build --target x86_64-unknown-uefi --release

# Build for AArch64 UEFI
cargo build --target aarch64-unknown-uefi --release

# Create bootable ISO
./scripts/build_uefi_iso.sh
```

## Testing

```bash
# Run in QEMU with OVMF
./scripts/run_uefi.sh

# Run with debug output
./scripts/run_uefi.sh --debug
```

## Security Model

### Secure Boot Chain

```
┌─────────────────────────────────────────────────────────────┐
│                    SECURE BOOT CHAIN                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌───────┐ │
│  │ Platform │───▶│   KEK    │───▶│    db    │───▶│ Helix │ │
│  │   Key    │    │          │    │          │    │ Boot  │ │
│  └──────────┘    └──────────┘    └──────────┘    └───────┘ │
│       │               │               │               │     │
│       ▼               ▼               ▼               ▼     │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌───────┐ │
│  │ Firmware │    │ Shim or  │    │  Boot    │    │Kernel │ │
│  │  ROM     │    │ Direct   │    │ Loader   │    │       │ │
│  └──────────┘    └──────────┘    └──────────┘    └───────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Measured Boot

```
┌─────────────────────────────────────────────────────────────┐
│                    MEASURED BOOT                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Component          │  PCR  │  Measurement                  │
│  ──────────────────────────────────────────────────────────  │
│  Firmware           │  0    │  SHA-256 of firmware code     │
│  Firmware Config    │  1    │  SHA-256 of config            │
│  Boot Loader        │  4    │  SHA-256 of helix-uefi        │
│  Boot Config        │  5    │  SHA-256 of boot config       │
│  Kernel             │  8    │  SHA-256 of kernel image      │
│  Kernel Cmdline     │  9    │  SHA-256 of command line      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Memory Model

### UEFI Memory Map → Rust Model

```
┌─────────────────────────────────────────────────────────────┐
│                    MEMORY REGIONS                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  0x0000_0000_0000_0000 ┌────────────────────────────────┐   │
│                        │        Reserved / Legacy        │   │
│  0x0000_0000_0010_0000 ├────────────────────────────────┤   │
│                        │      UEFI Boot Services         │   │
│                        │     (Reclaimable after exit)    │   │
│  0x0000_0000_xxxx_xxxx ├────────────────────────────────┤   │
│                        │      UEFI Runtime Services      │   │
│                        │    (Must preserve mappings)     │   │
│  0x0000_0000_xxxx_xxxx ├────────────────────────────────┤   │
│                        │         Usable RAM              │   │
│                        │    (Available for kernel)       │   │
│  0x0000_00xx_xxxx_xxxx ├────────────────────────────────┤   │
│                        │         MMIO Regions            │   │
│  0xFFFF_FFFF_FFFF_FFFF └────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Compatibility

| Firmware | Status |
|----------|--------|
| OVMF (QEMU) | ✅ Fully Tested |
| TianoCore EDK2 | ✅ Tested |
| AMI UEFI | 🔄 In Progress |
| InsydeH2O | 🔄 In Progress |
| Phoenix SecureCore | 📋 Planned |

## License

MIT OR Apache-2.0

## Contributing

See [CONTRIBUTING.md](../../docs/CONTRIBUTING.md)
