## Implementation Walkthrough:

1. cargo run
2. bootimage runner
3. QEMU starts
4. Legacy BIOS firmware executes
5. BIOS loads bootloader
6. Bootloader loads Genix Kernel

- Enters 64-bit long mode
- Configures paging
- Establishes initial memory mappings
- Prepares stack

7. Genix \_start()
8. Writes startup message to VGA memory (as of 10.10.2026: Loading GenixOS...)
9. Infinite loop
