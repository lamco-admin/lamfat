# lamfat

[![Crates.io](https://img.shields.io/crates/v/lamfat.svg)](https://crates.io/crates/lamfat)
[![Docs.rs](https://docs.rs/lamfat/badge.svg)](https://docs.rs/lamfat)
[![CI](https://github.com/lamco-admin/lamfat/actions/workflows/ci.yml/badge.svg)](https://github.com/lamco-admin/lamfat/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/lamco-admin/lamfat/blob/main/LICENSE-MIT)
[![Website](https://img.shields.io/badge/website-lamco.ai-blue.svg)](https://lamco.ai/products/lamboot/)

A `no_std` FAT12/16/32 filesystem library — read and write — for embedded and
UEFI-bootloader contexts that need to read a kernel (or write a config) on a FAT
volume without a userspace filesystem stack.

Faithful republish of [rafalh/rust-fatfs](https://github.com/rafalh/rust-fatfs).
See [`PROVENANCE.md`](https://github.com/lamco-admin/lamfat/blob/main/PROVENANCE.md)
for the exact upstream commit and the full list of changes from upstream.

- **Website:** [LamBoot](https://lamco.ai/products/lamboot/), the bootloader lamfat is published for
- **Documentation:** [docs.rs/lamfat](https://docs.rs/lamfat) and the [LamBoot docs](https://lamco.ai/products/lamboot/docs/)
- **Repository:** [github.com/lamco-admin/lamfat](https://github.com/lamco-admin/lamfat)
- **Company:** [Lamco Development LLC](https://lamco.ai/)

lamfat stays until upstream tags a `fatfs` 0.4.0 release on crates.io; LamBoot then moves to it.

## What it does

Given any seekable byte source (`IoBase + Read + Write + Seek`) holding a FAT
volume:

- Mounts the volume and exposes the root directory.
- Reads and writes files; creates and removes files and directories.
- Handles FAT12, FAT16, and FAT32, short (8.3) and long file names (`lfn`).

It is the same battle-tested `fatfs` engine, repackaged under a distinct crate
name so it can be a published dependency of the LamBoot bootloader family and
packaged for Debian/Fedora. The on-disk format logic is unchanged from upstream.

## Installation

```sh
cargo add lamfat
```

## Usage sketch

```rust
use lamfat::{FileSystem, FsOptions};

// `disk` is anything implementing lamfat's IoBase + Read + Write + Seek over the
// FAT volume's bytes — a block device, a partition, or (with the std feature) a
// file wrapped in a buffering stream.
fn list_root<D: lamfat::ReadWriteSeek>(disk: D) -> Result<(), lamfat::Error<D::Error>> {
    let fs = FileSystem::new(disk, FsOptions::new())?;
    for entry in fs.root_dir().iter() {
        let entry = entry?;
        let _name = entry.file_name();
    }
    Ok(())
}
```

Complete examples (`ls`, `cat`, `write`, `mkfatfs`, `partition`) are in the
[`examples/`](https://github.com/lamco-admin/lamfat/tree/main/examples) directory.

## Features

- `std` — enable `std::io` integration (off by default; the crate is `no_std`).
- `alloc` — long-filename and format paths that need allocation.
- `lfn` — long file name (VFAT) support.
- `unicode` — full Unicode case handling for file names.
- `log_level_error` / `_warn` / `_info` — compile-time log verbosity ceilings.

`chrono` (timestamps) is available as an opt-in dependency but is **off by
default**, so the default dependency tree is just `bitflags` + `log`.

## no_std

Set `default-features = false` for `no_std` operation. `alloc` is required for
long file names and formatting; the core read path works without it.

## Contributing

Issues and pull requests are welcome at
[github.com/lamco-admin/lamfat](https://github.com/lamco-admin/lamfat). Changes
that aren't specific to this republish belong upstream at
[rafalh/rust-fatfs](https://github.com/rafalh/rust-fatfs).

## License

MIT. See [`LICENSE-MIT`](LICENSE-MIT). Original copyright held by Rafał Harabień
(2017); republish-level modifications copyright Lamco Development LLC (2026). See
[`NOTICE`](NOTICE) and [`PROVENANCE.md`](PROVENANCE.md).
