# Operating System, made in Rust

Yet to unfold...

### Instructions
Install dependencies
- `$` `sudo apt install grub2 xorriso qemu-system`

Our compile target is a amd64 architecture, but since we are writing the OS from scratch, there is no underlying OS, system calls and such. We need to install a specific compile target, OS agnostic.
- `$` `rustup target add x86_64-unknown-none`

### Compilation and emulation
Builds the kernel ELF file
- `$` `cargo build`

Creates the bootable kernel image and emulates the os on QEMU
- `$` `cargo run`