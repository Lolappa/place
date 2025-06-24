use std::{
    ffi::OsStr,
    fs::File,
    io::{Error, ErrorKind, Result, Seek, SeekFrom, Write},
    path::Path,
};

use place_constants::*;

pub fn write_byte(x: usize, y: usize, value: u8) -> Result<()> {
    if x >= SIZE_X || y >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "position out of bounds",
        ));
    }
    let mut data_file = File::options()
        .write(true)
        .open(Path::new(LOCATION).join("data/data"))
        .unwrap();
    data_file.seek(SeekFrom::Start((y * SIZE_X + x).try_into().unwrap()))?;
    data_file.write(&[value])?;
    Ok(())
}

pub fn create_file(pos: (usize, usize), size: (usize, usize), name: &OsStr) -> Result<()> {
    if pos.0 >= SIZE_X || pos.1 >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "start position out of bounds",
        ));
    }
    if pos.0 + size.0 >= SIZE_X || pos.1 + size.1 >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "end position out of bounds",
        ));
    }

    let mut file = File::create_new(Path::new(LOCATION).join("data/files").join(name))?;
    file.write(&pos.0.to_ne_bytes())?;
    file.write(&pos.1.to_ne_bytes())?;
    file.write(&size.0.to_ne_bytes())?;
    file.write(&size.1.to_ne_bytes())?;
    Ok(())
}
