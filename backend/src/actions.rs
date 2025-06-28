use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, Error, ErrorKind, Seek, SeekFrom, Write},
    path::Path,
};

use place_constants::*;
use place_lib::fs::{Directory, File as PlaceFile, Position};

use crate::place_fs;

type Result = io::Result<()>;

pub fn write_byte(pos: Position, value: u8) -> Result {
    if pos.x() >= SIZE_X || pos.y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "position out of bounds",
        ));
    }
    let mut data_file = File::options()
        .write(true)
        .open(Path::new(DATA_LOCATION).join("data"))
        .unwrap();
    data_file.seek(SeekFrom::Start(
        (pos.y() * SIZE_X + pos.x()).try_into().unwrap(),
    ))?;
    data_file.write(&[value])?;
    Ok(())
}

pub fn create_file(file: PlaceFile, name: &OsStr) -> Result {
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

    let mut place_file = File::create_new(Path::new(DATA_LOCATION).join("file").join(name))?;
    place_file.write(&file.to_stdvec())?;

    place_fs::update_file_add(name)?;

    Ok(())
}

pub fn move_file(name: &OsStr, file: PlaceFile) -> Result {
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

    let mut place_file = File::options()
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_LOCATION).join("file").join(name))?;
    place_file.write(&file.to_stdvec())?;
    Ok(())
}

pub fn remove_file(file: &OsStr) -> Result {
    fs::remove_file(Path::new(DATA_LOCATION).join("file").join(file))?;
    Ok(())
}

pub fn rename_file(from: &OsStr, to: &OsStr) -> Result {
    let files_dir = Path::new(DATA_LOCATION).join("file");
    if !fs::exists(files_dir.join(to))? {
        fs::rename(files_dir.join(from), files_dir.join(to))?;
    }
    Ok(())
}

pub fn create_dir(dir: Directory, name: &OsStr) -> Result {
    if dir.start_pos().x() >= SIZE_X || dir.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "start position out of bounds",
        ));
    }
    if dir.end_pos().x() >= SIZE_X || dir.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "end position out of bounds",
        ));
    }

    let mut place_dir = File::create_new(Path::new(DATA_LOCATION).join("dir").join(name))?;
    place_dir.write(&dir.to_stdvec())?;

    place_fs::update_dir_add(name)?;

    Ok(())
}

pub fn move_dir(name: &OsStr, dir: Directory) -> Result {
    if dir.start_pos().x() >= SIZE_X || dir.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "start position out of bounds",
        ));
    }
    if dir.end_pos().x() >= SIZE_X || dir.end_pos().y() >= SIZE_Y {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "end position out of bounds",
        ));
    }

    let mut place_dir = File::options()
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_LOCATION).join("dir").join(name))?;
    place_dir.write(&dir.to_stdvec())?;
    Ok(())
}

pub fn remove_dir(dir: &OsStr) -> Result {
    fs::remove_file(Path::new(DATA_LOCATION).join("file").join(dir))?;
    Ok(())
}

pub fn rename_dir(from: &OsStr, to: &OsStr) -> Result {
    let dirs_dir = Path::new(DATA_LOCATION).join("dir");
    if !fs::exists(dirs_dir.join(to))? {
        fs::rename(dirs_dir.join(from), dirs_dir.join(to))?;
    }
    Ok(())
}
