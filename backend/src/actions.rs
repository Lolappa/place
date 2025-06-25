use std::{
    ffi::OsStr,
    fs::File,
    io::{Error, ErrorKind, Result, Seek, SeekFrom, Write},
    path::Path,
};

use place_constants::*;
use place_lib::file::{File as PlaceFile, Position};

pub fn write_byte(pos: Position, value: u8) -> Result<()> {
    if pos.x() >= SIZE_X || pos.y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "position out of bounds",
        ));
    }
    let mut data_file = File::options()
        .write(true)
        .open(Path::new(LOCATION).join("data/data"))
        .unwrap();
    data_file.seek(SeekFrom::Start(
        (pos.y() * SIZE_X + pos.x()).try_into().unwrap(),
    ))?;
    data_file.write(&[value])?;
    Ok(())
}

pub fn create_file(file: PlaceFile, name: &OsStr) -> Result<()> {
    if file.start_pos().x() >= SIZE_X || file.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "start position out of bounds",
        ));
    }
    if file.end_pos().x() >= SIZE_X || file.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "end position out of bounds",
        ));
    }

    let mut place_file = File::create_new(Path::new(LOCATION).join("data/files").join(name))?;
    place_file.write(&file.to_stdvec())?;
    Ok(())
}
