# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.3] - 2026-10-15

### Fixed

- Five `assert!`/`debug_assert!` equality checks are written as `assert_eq!`
  and `debug_assert_eq!`, which Rust 1.99's clippy (`manual_assert_eq`)
  requires; CI failed on them. Behavior is unchanged.

### Changed

- The description names Lamco Development LLC, the README gains website,
  documentation, install and contributing sections, and CI also runs weekly.

## [0.4.2] — 2026-06-08

### Changed

- Drop the `fscommon` dev-dependency from the tests and examples so the crate's
  test build needs no out-of-tree helper, easing Debian/Fedora packaging
  (`fscommon` is not in those archives). `BufStream` was only a buffering
  convenience; `FileSystem::new` accepts any `Read + Write + Seek` via
  `IntoStorage`, so tests and examples now pass the `File`/`Cursor` directly. The
  `partition` example (the sole `fscommon::StreamSlice` user) was removed;
  partition handling remains documented on `FileSystem::new`.
- Bump the `env_logger` dev-dependency to 0.11, which carries the archive's
  current version and raises the crate MSRV to 1.71 (`env_logger 0.11`'s floor).

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
