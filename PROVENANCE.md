# lamfat — Provenance

This crate is a faithful republish of the upstream `fatfs` crate under a distinct
name, so it can be a published crates.io dependency of the LamBoot bootloader
family and packaged for Debian/Fedora (debcargo).

## Upstream

- **Project:** rust-fatfs
- **Repository:** https://github.com/rafalh/rust-fatfs
- **License:** MIT
- **Original author:** Rafał Harabień
- **Vendored commit:** `c4b88477b22ca7e5131fbd8891f62a5deaa88e6e` (the in-tree
  0.4.0 `no_std` git line)

The upstream copyright is preserved in `LICENSE-MIT` (Rafał Harabień, 2017) with
the Lamco line added beneath it for the republish-level changes.

## Why we republished

- The `no_std` `IoBase`/`Read`/`Write`/`Seek` trait surface that LamBoot needs
  exists only on the 0.4.0 **git** line; the published crates.io `fatfs` 0.3.6 is
  `std`/`core_io`-based and predates that surface.
- LamBoot's already-working read-only FAT backend is built on this engine;
  publishing it as a named crate lets the bootloader depend on a crates.io
  release rather than a git pin, and lets debcargo package it for Debian main.
- Distinct crate name (`lamfat` ≠ `fatfs`) so the two never collide on crates.io.

## Changes from upstream

| Area | Change | Rationale |
|---|---|---|
| Crate name | `fatfs` → `lamfat` (package + `[lib] name`); crate self-references in module docs, examples, and tests updated to match | Distinct published name; doctests/examples must reference the real crate |
| Default features | `chrono` removed from `default` (still available as an opt-in feature) | Keeps `chrono`'s transitive tree out of Debian-main scope; the default tree is just `bitflags` + `log`, both MIT/Apache and in Debian main |
| Build hygiene | Added `README.md`, `PROVENANCE.md`, `NOTICE`, `CHANGELOG.md`, dual-copyright `LICENSE-MIT`, CI workflow, `deny.toml`, and the Lamco `rustfmt`/`clippy` config; reformatted to the Lamco `rustfmt` style | Upstream git line shipped none of these; required by the Lamco crate standard |

**The on-disk FAT format logic and the read/write engine are unchanged from
upstream.** No source-logic changes were made.

## What we did NOT change

- The FAT12/16/32 parser and writer — the on-disk format is stable and needs no
  changes from upstream.
- Feature semantics, other than moving `chrono` out of `default`.
- The public API surface (`FileSystem`, `Dir`, `File`, `FsOptions`, the I/O
  traits) — drop-in compatible with the upstream 0.4.0 git line.

## Verification

The full upstream test suite passes on the host (the `fsck` integration tests
require `dosfstools` / `fsck.vfat` on `PATH`). CI builds the library for a
bare-metal `no_std` target (`thumbv7em-none-eabihf`) to prove no `std` leaked in.

## Maintenance policy

- We own this republish. Republish-level fixes (packaging, hygiene, dependency
  currency) land here.
- Published to crates.io as `lamfat` — a maintained republish, distinct in name
  from the upstream `fatfs` crate.
- We may pull selected upstream fixes from the rust-fatfs git line as they land;
  this is positioned as a maintained republish, not original work.
