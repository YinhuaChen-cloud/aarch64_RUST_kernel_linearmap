# Minimal AArch64 Rust kernel

This project builds a minimal bare-metal kernel for `qemu-system-aarch64` with GN + Ninja and prints `hello world` through the PL011 UART on the QEMU `virt` machine.

## Files

- `src/main.rs`: reset entry and Rust kernel code
- `linker.ld`: fixed load address and memory layout
- `BUILD.gn`: GN target that invokes `rustc`
- `build/build_kernel.py`: tiny build helper used by Ninja
- `tools/run_qemu.sh`: helper to boot the generated ELF in QEMU

## Prerequisites

You need these tools in `PATH`:

- `gn`
- `ninja`
- `rustc`
- `qemu-system-aarch64`

Install the Rust target used by the build:

```bash
rustup target add aarch64-unknown-none
```

## Build

```bash
gn gen out/default
ninja -C out/default
```

The build output is:

- `out/default/kernel.elf`

## Run

```bash
bash tools/run_qemu.sh
```

Expected serial output:

```text
hello world
```

## Manual QEMU command

```bash
qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -nographic \
  -serial mon:stdio \
  -smp 1 \
  -kernel out/default/kernel.elf
```
