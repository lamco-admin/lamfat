use std::{env, fs, io};

use lamfat::{format_volume, FormatVolumeOptions, StdIoWrapper};

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("image path expected");
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
    format_volume(&mut StdIoWrapper::from(file), FormatVolumeOptions::new())?;
    Ok(())
}
