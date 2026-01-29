# Helix OS Framework - Roadmap

## Vision

Helix is not an operating system — it's a **framework for creating operating systems**.

The goal is to provide a highly modular, policy-free kernel where every major component
(scheduler, allocator, filesystem, etc.) can be replaced at runtime without rebooting.

---

## Phase 1: Foundation (Months 1-6)

### 1.1 Boot Infrastructure
- [x] Multiboot2 bootloader support ✅ (2025-01-28)
- [x] Limine protocol support ✅ (2025-01-29)
  - [x] Protocol layer (magic, request IDs, raw structures)
  - [x] All 18 request types (bootloader info, memory map, HHDM, SMP, framebuffer, etc.)
  - [x] Safe response wrappers
  - [x] Memory management (PhysAddr, VirtAddr, HHDM translation)
  - [x] CPU utilities (SMP, per-CPU data, barriers)
  - [x] Advanced framebuffer (Console, Graphics, double buffering)
  - [x] Firmware support (ACPI, SMBIOS, EFI, DTB)
  - [x] File/module loading (CPIO, ELF parsing)
  - [x] Multi-architecture (x86_64, aarch64, riscv64)
- [x] UEFI boot support ✅ (2026-01-28) - **134,673 lignes, 144 fichiers, 70+ modules**
  - [x] **Core UEFI** (raw types, GUIDs, status codes, handles)
  - [x] **Services** (Boot Services, Runtime Services, System Table)
  - [x] **Protocols** (GOP, File, Block I/O, Network, USB, PCI, Serial)
  - [x] **Memory** (allocator, page tables, virtual mapping, pool allocator)
  - [x] **Handoff** (boot info structure pour le kernel)
  - [x] **Multi-architecture** (x86_64, aarch64)
  - [x] **Security** (Secure Boot, TPM 2.0, crypto, signature verification)
  - [x] **Filesystems** (FAT12/16/32 complet, lecture/écriture, LFN)
  - [x] **Partitions** (GPT, MBR, parsing complet)
  - [x] **Binary Loaders** (ELF64, PE/COFF, relocations)
  - [x] **System Tables** (ACPI complet avec FADT/MADT/DSDT, SMBIOS)
  - [x] **Cryptographie** (SHA-256, RSA, HMAC, vérification de signatures)
  - [x] **Network Boot** (PXE, DHCP, TFTP, HTTP/HTTPS)
  - [x] **Console** (texte, graphique, framebuffer, fonts PSF)
  - [x] **Menu système** (navigation, sélection, configuration)
  - [x] **Graphics** (primitives 2D, double buffering, sprites)
  - [x] **Configuration** (parsing TOML/INI, boot entries)
  - [x] **Boot Manager** (gestion multi-boot, fallback, chainload)
  - [x] **Theme Engine** (personnalisation UI, couleurs, layouts)
  - [x] **Splash Screen** (logo animé, barre de progression)
  - [x] **Help System** (aide contextuelle, documentation intégrée)
  - [x] **Recovery Mode** (diagnostic, réparation, shell de secours)
  - [x] **Orchestrator** (séquencement du boot, gestion d'erreurs)
  - [x] **Validation** (vérification intégrité, checksums)
  - [x] **System Info** (détection hardware, reporting)
  - [x] **Performance** (mesures timing, profiling boot)
- [x] Early console (serial, VGA) ✅ (2026-01-28)
- [x] Memory map parsing ✅ (2026-01-28)
- [ ] Kernel relocation

### 1.2 Hardware Abstraction Layer
- [x] HAL trait definitions
- [ ] x86_64 implementation
  - [ ] GDT/IDT setup
  - [ ] Paging (4-level, 5-level)
  - [ ] APIC/IOAPIC
  - [ ] TSC/HPET timers
- [ ] aarch64 implementation
  - [ ] Exception levels
  - [ ] MMU (4KB/64KB pages)
  - [ ] GIC
- [ ] riscv64 implementation (basic)

### 1.3 Core Kernel
- [x] Orchestrator design
- [x] Capability broker
- [x] Resource broker
- [x] Panic handler
- [ ] Early boot sequence
- [ ] Subsystem initialization order

### 1.4 Memory Subsystem
- [x] Physical allocator framework
- [x] Bitmap allocator
- [x] Buddy allocator
- [x] Virtual memory framework
- [ ] Kernel heap (working)
- [ ] On-demand paging

### 1.5 Execution Subsystem
- [x] Scheduler framework
- [x] Thread abstraction
- [x] Process abstraction
- [x] Round-robin scheduler module
- [ ] Context switching (per-arch)
- [ ] Idle thread
- [ ] Basic SMP support

### 1.6 Module System
- [x] Module trait
- [x] Module loader framework
- [x] Dependency resolution
- [x] Hot reload framework
- [ ] ELF loader (working)
- [ ] Module verification

---

## Phase 2: Core Features (Months 7-12)

### 2.1 IPC / Message Bus
- [ ] Synchronous message passing
- [ ] Asynchronous channels
- [ ] Shared memory
- [ ] Signals
- [ ] Event system

### 2.2 Security Subsystem
- [x] Secure Boot integration ✅ (2026-01-28) - Via UEFI bootloader
- [x] TPM 2.0 support ✅ (2026-01-28)
- [x] Cryptographic primitives ✅ (2026-01-28) - SHA-256, RSA, HMAC
- [ ] Capability refinement
- [ ] MAC framework
- [ ] Sandboxing
- [ ] Audit logging

### 2.3 I/O Subsystem
- [x] Block device framework ✅ (2026-01-28) - Via UEFI Block I/O
- [x] Character device framework ✅ (2026-01-28) - Via UEFI Serial I/O
- [ ] VFS framework
- [ ] DMA support
- [ ] Interrupt routing

### 2.4 Time Subsystem
- [ ] System clock
- [ ] Timers
- [ ] Watchdog
- [ ] RTC integration

### 2.5 Additional Schedulers
- [ ] CFS (Completely Fair Scheduler)
- [ ] Real-time scheduler (FIFO/RR)
- [ ] Cooperative scheduler
- [ ] Deadline scheduler

### 2.6 Additional Allocators
- [ ] TLSF allocator
- [ ] Slab allocator (full)
- [ ] Zone allocator

### 2.7 Filesystems
- [x] FAT12/16/32 (read/write) ✅ (2026-01-28) - Complet avec LFN
- [ ] RamFS
- [ ] DevFS
- [ ] ProcFS
- [ ] Basic ext2 (read-only)

---

## Phase 3: Userland (Months 13-18)

### 3.1 System Call Interface
- [ ] Syscall ABI stabilization
- [ ] POSIX subset implementation
- [ ] Custom Helix syscalls
- [ ] Syscall filtering

### 3.2 Process Management
- [ ] fork/exec
- [ ] Signals (full)
- [ ] Process groups
- [ ] Sessions

### 3.3 User Space
- [ ] ELF loading (user)
- [ ] Dynamic linking
- [ ] Thread-local storage
- [ ] User-space allocator

### 3.4 Shell & Utilities
- [ ] Basic shell
- [ ] Core utilities (ls, cat, etc.)
- [ ] Process viewer

---

## Phase 4: Ecosystem (Months 19-24)

### 4.1 SDK & Tooling
- [ ] `helix-build` - Build profiles
- [ ] `helix-pack` - Package modules
- [ ] `helix-test` - Testing framework
- [ ] Module templates
- [ ] Documentation generator

### 4.2 Additional Profiles
- [ ] Desktop profile (with graphics)
- [ ] Server profile (networking)
- [ ] Embedded profile (minimal)
- [ ] Secure profile (hardened)

### 4.3 Drivers
- [ ] VirtIO (block, net, console)
- [ ] PS/2 keyboard
- [ ] Serial console
- [ ] Framebuffer

### 4.4 Networking (optional)
- [x] Network boot framework ✅ (2026-01-28) - PXE, TFTP, HTTP/HTTPS, DHCP
- [ ] Network stack framework
- [ ] TCP/IP (as module)
- [ ] Sockets

### 4.5 Graphics (optional)
- [x] Framebuffer abstraction ✅ (2026-01-28) - Via UEFI GOP
- [x] 2D graphics library ✅ (2026-01-28) - Primitives, sprites, double buffering
- [ ] Window system interface

---

## Milestones

| Milestone | Target Date | Description | Status |
|-----------|-------------|-------------|--------|
| M0 | Month 1 | Boot to serial output | ✅ Completed (2026-01-28) |
| M1 | Month 3 | Memory management working | 🔄 In Progress |
| M2 | Month 6 | Scheduler with context switching | ⏳ Pending |
| M3 | Month 9 | First module hot-reload | ⏳ Pending |
| M4 | Month 12 | Basic file system | 🔄 In Progress (FAT32 done) |
| M5 | Month 15 | First user process | ⏳ Pending |
| M6 | Month 18 | Shell running | ⏳ Pending |
| M7 | Month 24 | SDK release | ⏳ Pending |

---

## Recent Achievements

### 2026-01-28: UEFI Bootloader Complete 🎉
Implémentation complète du bootloader UEFI avec:
- **134,673 lignes de code Rust** (100% no_std)
- **144 fichiers source**
- **70+ modules**
- Support multi-architecture (x86_64, aarch64)
- Aucune dépendance externe

Fonctionnalités principales:
- Boot complet via UEFI avec handoff au kernel
- Secure Boot et TPM 2.0
- FAT32 complet (lecture/écriture/LFN)
- Network Boot (PXE, HTTP/HTTPS, TFTP)
- Interface graphique (menus, thèmes, splash screens)
- Mode recovery et diagnostics
- Parsers binaires (ELF64, PE/COFF)
- Tables système (ACPI, SMBIOS)

---

## Success Criteria

1. **Modularity**: Any major component can be replaced without reboot
2. **Hot Reload**: Scheduler/allocator swap with < 10ms downtime
3. **Policy-Free**: Zero hard-coded policies in kernel core
4. **Multi-Arch**: Same codebase for x86_64, aarch64, riscv64
5. **Documentation**: Every public API documented
6. **Testing**: > 80% test coverage on core components

---

## Non-Goals (for v1.0)

- GUI/Desktop environment
- Real hardware driver coverage (focus on VirtIO)
- Full POSIX compliance
- Binary compatibility with Linux
- Production readiness

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Priority areas for contributions:
1. Architecture-specific HAL implementations
2. Scheduler/allocator modules
3. Filesystem modules
4. Documentation and examples
