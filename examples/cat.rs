use std::{
    env,
    fs::File,
    io::{self, prelude::*},
};

use fscommon::BufStream;
use lamfat::{FileSystem, FsOptions};

fn main() -> io::Result<()> {
    let file = File::open("resources/fat32.img")?;
    let buf_rdr = BufStream::new(file);
    let fs = FileSystem::new(buf_rdr, FsOptions::new())?;
    let root_dir = fs.root_dir();
    let mut file = root_dir.open_file(&env::args().nth(1).expect("filename expected"))?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    print!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}
