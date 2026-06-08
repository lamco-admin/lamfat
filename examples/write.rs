use std::{
    fs::OpenOptions,
    io::{self, prelude::*},
};

use lamfat::{FileSystem, FsOptions};

fn main() -> io::Result<()> {
    let img_file = match OpenOptions::new().read(true).write(true).open("fat.img") {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open image!");
            return Err(err);
        }
    };
    let options = FsOptions::new().update_accessed_date(true);
    let fs = FileSystem::new(img_file, options)?;
    let mut file = fs.root_dir().create_file("hello.txt")?;
    file.write_all(b"Hello World!")?;
    Ok(())
}
