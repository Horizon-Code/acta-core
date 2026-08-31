# Local Build Environment

## Status

Operational note, not a requirement of the project. Records what one WSL2 machine needed on
2026-08-31 so it does not become a hidden dependency for whoever inherits the environment.

## The situation

The machine had **no Rust toolchain and no C toolchain**: `cargo`, `rustc` and `cc` were all
absent, `sudo` required a password, and `apt install` was therefore unavailable.

Without a linker, `cargo test --workspace` cannot run: the proc-macro build scripts of `serde`
and friends need to link host binaries.

## What was done

Both steps are user-local and reversible. Neither touches the system, and neither is committed
to the repository.

1. **Rust**: `rustup` installed into `~/.cargo` and `~/.rustup` (minimal profile, stable), plus
   the `rustfmt` component.
2. **C linker**: the Ubuntu packages for `gcc-12`, `cpp-12`, `binutils`, `libc6-dev`,
   `linux-libc-dev` and `libgcc-12-dev` downloaded with `apt-get download` (no `sudo` needed to
   download), extracted with `dpkg-deb -x` into a scratch prefix, and fronted by a `cc` wrapper
   script that passes `--sysroot` at that prefix. Runtime libraries are symlinked back to the
   system copies.

`cargo fmt --all` and `cargo test --workspace` then run normally.

## What this means for anyone else

- **Nothing in the repository depends on this.** CI (`.github/workflows/ci.yml`) uses
  `dtolnay/rust-toolchain@stable` on a GitHub runner that already has a C toolchain.
- On a normal developer machine the equivalent is `rustup` plus `build-essential`, installed
  once through the system package manager.
- The scratch prefix is temporary. If a session cannot build, this is the thing to re-check
  first — the failure presents as `error: linker 'cc' not found`, which reads like a Rust
  problem and is not one.
