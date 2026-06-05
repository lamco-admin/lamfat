# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] — 2026-06-05

### Added

- Initial release: a faithful republish of `fatfs`
  (`rafalh/rust-fatfs` @ `c4b88477`, the `no_std` 0.4.0 git line) under the
  `lamfat` name, so it can be a published crates.io dependency of the LamBoot
  bootloader family and packaged for Debian/Fedora. `no_std` FAT12/16/32, read
  and write, short and long file names.

### Changed

- `chrono` removed from the default feature set (still available as an opt-in
  feature) so the default dependency tree is just `bitflags` + `log`.

See [`PROVENANCE.md`](PROVENANCE.md) for the exact upstream commit and the full
list of changes from upstream.
