## Development

GenixOS is an experimental operating system written in Rust.

The kernel targets bare-metal x86_64 hardware and currently uses
the Rust bootloader crate (0.9.x) for legacy BIOS booting.

### Requirements

- Rust **nightly** toolchain
- Cargo
- Rust source component (`rust-src`)
- Bootimage
- QEMU (x86_64)
- GNU Make (optional)

### Building

Compile the Genix kernel:

```bash
cargo build
```

Create a bootable image:

```bash
cargo bootimage
```

### Running

Build and boot GenixOS in QEMU:

```bash
cargo run
```

Alternatively, use:

```bash
make run
```

Both commands are supported by the development environment

### Clean Build

Remove existing build artifacts:

```bash
cargo clean
```

Rebuild the kernel and boot image:

```bash
cargo build
cargo bootimage
```

### Boot Verification

To verify reproducible booting:

1. Run `cargo run`.
2. Verify that QEMU starts successfully.
3. Confirm that `Hellow World!` appears on screen.
4. Close QEMU.
5. Repeat until five consecutive boots have succeeded.

### Current Limitations

GenixOS is in early development.

Currently, the kernel:

- Uses a custom x86_64 target.
- Runs without the Rust standart library.
- Uses the bootloader crate for early intialization.
- Writes text directly to VGA memory.
- Enters an infinite loop after initialization.

There is currently no memory allocator, process scheduler,
filesystem, or interactive shell.

## Kernel Safety Assumptions

Genix OS currently targets x86_64 under QEMU.

The bootloader establishes the initial memory mappings, page tables,
and kernel stack before transferring control to the kernel.

The VGA driver assumes that virtual address 0xB8000 maps to accessible
VGA text memory.

Hardware I/O is encapsulated in architecture-specific functions and
device drivers. Global console and serial state is protected by spinlocks.

Spinlocks are non-reentrant and currently do not disable interrupts.
Interrupt safety and multicore sync require further review as the
kernel evolves.
