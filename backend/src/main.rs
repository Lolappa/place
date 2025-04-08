use place_constants::*;
use place_lib::commands::Command;
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{prelude::*, Error, ErrorKind, Result, SeekFrom},
    os::unix::{
        ffi::OsStringExt,
        net::{UnixListener, UnixStream},
    },
    path::Path,
    thread, usize,
};

fn main() {
    create_data();
    write_byte(4, 31, 50);
    create_file((2, 2), (4, 4), &OsString::from("tst"));
    create_file((2, 2), (0, 0), &OsString::from("empty"));
    if let Ok(listener) = UnixListener::bind(Path::new(LOCATION).join(".sock")) {
        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    thread::spawn(|| handle_client(stream));
                }
                Err(_err) => {
                    /* connection failed */
                    break;
                }
            }
        }
    }
}

fn handle_client(mut stream: UnixStream) {
    let mut buf = [0u8; 16 + size_of::<u32>() + 1];

    if let Ok(len) = stream.read(&mut buf) {
        if len == buf.len() {
            // Get values from buffer
            let slice = &buf[..];
            let (token, slice): ([u8; 16], &[u8]) = {
                let (temp, slice) = slice.split_at(16);
                (temp.try_into().unwrap(), slice)
            };
            let (userid, slice): (u32, &[u8]) = {
                let (temp, slice) = slice.split_at(size_of::<u32>());
                (u32::from_ne_bytes(temp.try_into().unwrap()), slice)
            };
            let command = if let Ok(value) = slice[0].try_into() {
                value
            } else {
                todo!();
            };

            if can_do_change(token, userid) {
                // Ask client for more information about request
                if let Err(err) = stream.write(&[0]) {
                    todo!();
                };
                use Command::*;
                match command {
                    SetByte => {
                        let mut buf = [0u8; 2 * size_of::<usize>() + 1];
                        if let Ok(len) = stream.read(&mut buf) {
                            if len == buf.len() {
                                let slice = &buf[..];
                                let (x, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let (y, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let byte = slice[0];

                                if let Err(err) = write_byte(x, y, byte) {
                                    todo!()
                                } else {
                                    set_timestamp(userid);
                                    if let Err(err) = stream.write(&[0]) {
                                        todo!()
                                    }
                                }
                            }
                        }
                    }
                    CreateFile => {
                        let mut buf = [0u8; 4 * size_of::<usize>()];
                        if let Ok(len) = stream.read(&mut buf) {
                            if len == buf.len() {
                                let slice = &buf[..];
                                let (x, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let (y, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let (dx, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let (dy, _slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };

                                if let Err(err) = stream.write(&[0]) {
                                    todo!()
                                } else {
                                    // Get filename input
                                    let mut buf = [0; 32];
                                    let mut filename = OsString::new();
                                    loop {
                                        if let Ok(len) = stream.read(&mut buf) {
                                            match len {
                                                0 => {
                                                    if filename.len() != 0 {
                                                        break;
                                                    }
                                                }
                                                32 => {
                                                    filename.push(OsString::from_vec(buf.to_vec()));
                                                }
                                                len => {
                                                    filename.push(OsString::from_vec(
                                                        buf[..len].to_vec(),
                                                    ));
                                                    break;
                                                }
                                            }
                                        } else {
                                            todo!()
                                        }
                                    }

                                    if let Err(err) = create_file((x, y), (dx, dy), &filename) {
                                        todo!()
                                    } else {
                                        set_timestamp(userid);
                                        if let Err(err) = stream.write(&[0]) {
                                            todo!()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    RemoveFile => {
                        //??
                        todo!()
                    }
                    RenameFile => {
                        //??
                        todo!()
                    }
                    MoveFile => {
                        let mut buf = [0u8; 2 * size_of::<usize>()];
                        if let Ok(len) = stream.read(&mut buf) {
                            if len == buf.len() {
                                let slice = &buf[..];
                                let (x, slice) = {
                                    let (temp, slice) = slice.split_at(size_of::<usize>());
                                    (usize::from_ne_bytes(temp.try_into().unwrap()), slice)
                                };
                                let y = usize::from_ne_bytes(slice.try_into().unwrap());
                                //??
                                todo!()
                            }
                        }
                    }
                }
                set_timestamp(userid);
                if let Err(err) = stream.write(&[0]) {
                    todo!()
                };
            } else {
                // Tell client about failed request
                if let Err(err) = stream.write(&[1]) {
                    todo!()
                };
            }
        } else {
            todo!()
        }
    }
}

fn can_do_change(token: [u8; 16], userid: u32) -> bool {
    todo!()
}

fn set_timestamp(userid: u32) {
    todo!()
}

fn create_data() {
    let data_file = Path::new(LOCATION).join("data/data");

    if data_file.exists() != true {
        let mut data_file = File::create(data_file).unwrap();
        let _ = data_file.write_all(&[0u8; SIZE_X * SIZE_Y]);
    }
}

fn write_byte(x: usize, y: usize, value: u8) -> Result<()> {
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

fn create_file(pos: (usize, usize), size: (usize, usize), name: &OsStr) -> Result<()> {
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
