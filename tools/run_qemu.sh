#!/usr/bin/env bash
set -euo pipefail

BUILD_DIR="${1:-out/default}"

exec qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -m 1536M \
  -nographic \
  -serial mon:stdio \
  -smp 1 \
  -kernel "${BUILD_DIR}/kernel.elf" \
  -s -S
