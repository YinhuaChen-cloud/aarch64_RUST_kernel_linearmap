#!/usr/bin/env python3
import argparse
import pathlib
import shutil
import subprocess
import sys


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build the bare-metal AArch64 Rust kernel")
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--early-source", required=True)
    parser.add_argument("--later-source", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--cfg", action="append", default=[])
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    early_source = pathlib.Path(args.early_source).resolve()
    later_source = pathlib.Path(args.later_source).resolve()
    linker = pathlib.Path(args.linker).resolve()
    output = pathlib.Path(args.output).resolve()
    later_rlib = output.parent / "liblater.rlib"
    output.parent.mkdir(parents=True, exist_ok=True)

    rustc = shutil.which(args.rustc)
    if rustc is None:
        print(
            "error: rustc was not found. Install Rust and add the aarch64-unknown-none target:\n"
            "  rustup target add aarch64-unknown-none",
            file=sys.stderr,
        )
        return 1

    common_command = [
        rustc,
        "--edition=2021",
        "-C",
        "opt-level=z",
        "-C",
        "panic=abort",
        "-C",
        "relocation-model=static",
        "--target",
        "aarch64-unknown-none",
    ]

    later_command = [
        *common_command,
        "--crate-name",
        "later",
        "--crate-type",
        "rlib",
    ]

    for cfg in args.cfg:
        later_command.extend(["--cfg", cfg])

    later_command.extend([
        str(later_source),
        "-o",
        str(later_rlib),
    ])

    command = [
        *common_command,
        "--crate-name",
        "kernel",
        "--crate-type",
        "bin",
        "-C",
        f"link-arg=-T{linker}",
        "--extern",
        f"later={later_rlib}",
    ]

    for cfg in args.cfg:
        command.extend(["--cfg", cfg])

    command.extend([
        str(early_source),
        "-o",
        str(output),
    ])

    print(" ".join(later_command))
    subprocess.run(later_command, check=True)

    print(" ".join(command))
    subprocess.run(command, check=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
