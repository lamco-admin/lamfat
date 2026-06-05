//! A FAT filesystem library implemented in Rust.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/lamfat) and can be
//! used by adding `lamfat` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! lamfat = "0.4"
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use std::io::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     # std::fs::copy("resources/fat16.img", "tmp/fat.img")?;
//!     // Initialize a filesystem object
//!     let img_file = std::fs::OpenOptions::new().read(true).write(true)
//!         .open("tmp/fat.img")?;
//!     let buf_stream = fscommon::BufStream::new(img_file);
//!     let fs = lamfat::FileSystem::new(buf_stream, lamfat::FsOptions::new())?;
//!     let root_dir = fs.root_dir();
//!
//!     // Write a file
//!     root_dir.create_dir("foo")?;
//!     let mut file = root_dir.create_file("foo/hello.txt")?;
//!     file.truncate()?;
//!     file.write_all(b"Hello World!")?;
//!
//!     // Read a directory
//!     let dir = root_dir.open_dir("foo")?;
//!     for r in dir.iter() {
//!         let entry = r?;
//!         println!("{}", entry.file_name());
//!     }
//!     # std::fs::remove_file("tmp/fat.img")?;
//!     # Ok(())
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Disable warnings to not clutter code with cfg too much
#![cfg_attr(
    not(all(feature = "alloc", feature = "lfn")),
    allow(dead_code, unused_imports)
)]

extern crate log;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[macro_use]
mod log_macros;

mod boot_sector;
mod dir;
mod dir_entry;
mod error;
mod file;
mod fs;
mod io;
mod table;
mod time;

pub use crate::{dir::*, dir_entry::*, error::*, file::*, fs::*, io::*, time::*};
