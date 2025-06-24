use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};
use place_backend::*;
use place_constants::*;
use place_lib::{commands::Command, packet::Packet};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::File,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, SystemTimeError},
};
use users::uid_t;

fn main() {
    create_data();
    // NOTE: debug actions
    let _ = actions::write_byte(4, 31, 50);
    let _ = actions::create_file((2, 2), (4, 4), &OsString::from("tst"));
    let _ = actions::create_file((2, 2), (0, 0), &OsString::from("empty"));

    let timestamps = Arc::new(Mutex::new(HashMap::<uid_t, SystemTime>::new()));
    if let Ok(listener) = UnixListener::bind(SOCK_LOCATION) {
        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    let timestamps = Arc::clone(&timestamps);
                    thread::spawn(move || handle_client(stream, &timestamps));
                }
                Err(_err) => { /* connection failed */ }
            }
        }
    }
}

fn handle_client(mut stream: UnixStream, timestamps: &Mutex<HashMap<uid_t, SystemTime>>) {
    let crypt = ChaCha20Poly1305::new(&[0u8; 32].into());
    // TODO: use include_bytes!() or something here
    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

    if let Err(err) = stream.write_all(&nonce) {
        todo!()
    };

    let mut size_of_packet = [0u8; size_of::<usize>()];
    if let Err(err) = stream.read_exact(&mut size_of_packet) {
        todo!()
    };
    let size_of_packet = usize::from_ne_bytes(size_of_packet);

    let mut ciphertext = Vec::<u8>::with_capacity(size_of_packet);
    let mut buf = [0u8; 256];
    while ciphertext.len() != size_of_packet {
        if let Ok(bytes) = stream.read(&mut buf) {
            ciphertext.extend_from_slice(&buf[..bytes]);
        };
    }

    let packet = match Packet::from_bytes(
        if let Ok(value) = crypt.decrypt(&nonce, &ciphertext[..]) {
            value
        } else {
            todo!()
        }
        .as_slice(),
    ) {
        Ok(value) => value,
        Err(_) => todo!(),
    };

    let uid = packet.header().uid();
    if let Ok(can_change) = can_do_change(uid, timestamps) {
        if can_change {
            match packet.header().command() {
                Command::SetByte => {
                    if let Some(block) = packet.blocks().get(0) {
                        let mut slice = block.content();
                        if slice.len() != size_of::<usize>() + size_of::<usize>() + 1 {
                            todo!()
                        }

                        let x = {
                            let tmp: &[u8];
                            (tmp, slice) = slice.split_at(size_of::<usize>());
                            usize::from_ne_bytes(tmp.try_into().unwrap())
                        };

                        let y = {
                            let tmp: &[u8];
                            (tmp, slice) = slice.split_at(size_of::<usize>());
                            usize::from_ne_bytes(tmp.try_into().unwrap())
                        };

                        let value = slice[0];
                        if let Err(err) = actions::write_byte(x, y, value) {
                            todo!()
                        };
                    }
                }
                Command::CreateFile => {}
                Command::RemoveFile => {}
                Command::RenameFile => {}
                Command::MoveFile => {}
            }
            set_timestamp(uid, timestamps);
        }
    }
}

fn can_do_change(
    userid: uid_t,
    timestamps: &Mutex<HashMap<uid_t, SystemTime>>,
) -> Result<bool, SystemTimeError> {
    if let Ok(timestamps) = timestamps.lock() {
        if let Some(timestamp) = timestamps.get(&userid) {
            match timestamp.elapsed() {
                Ok(time) => Ok(time > Duration::from_secs(300)),
                Err(error) => Err(error),
            }
        } else {
            Ok(true)
        }
    } else {
        todo!()
    }
}

fn set_timestamp(userid: uid_t, timestamps: &Mutex<HashMap<uid_t, SystemTime>>) {
    if let Ok(mut timestamps) = timestamps.lock() {
        if let Some(timestamp) = timestamps.get_mut(&userid) {
            *timestamp = SystemTime::now();
        };
    } else {
        todo!()
    }
    // TODO: Save hashmap in case the server crashes
}

fn create_data() {
    let data_file = Path::new(LOCATION).join("data/data");

    if data_file.exists() != true {
        let mut data_file = File::create(data_file).unwrap();
        let _ = data_file.write_all(&[0u8; SIZE_X * SIZE_Y]);
    }
}
/*
fn read_vec(mut stream: &UnixStream) -> Result<Vec<u8>> {
    // Get length of Vec
    let mut buf = [0u8; size_of::<usize>()];
    stream.read_exact(&mut buf)?;
    let length = usize::from_ne_bytes(buf);

    let mut vector: Vec<u8> = vec![];
    // Get Vec from stream
    let mut buf = [0u8; 32];
    loop {
        let len = stream.read(&mut buf)?;
        vector.extend_from_slice(&buf[..len]);
        if length == vector.len() {
            break;
        };
    }
    Ok(vector)
}*/

/*
fn handle_client(mut stream: UnixStream) {
    let mut buf = [0u8; size_of::<[u8; 16]>() + size_of::<uid_t>() + 1];

    if let Err(err) = stream.read_exact(&mut buf) {
        todo!()
    } else {
        // Get values from buffer
        let slice = &buf[..];
        let (token, slice): ([u8; 16], &[u8]) = {
            let (temp, slice) = slice.split_at(16);
            (temp.try_into().unwrap(), slice)
        };
        let (userid, slice): (uid_t, &[u8]) = {
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
                                let filename: OsString;
                                if let Ok(vec) = read_vec(&stream) {
                                    filename = OsString::from_vec(vec);
                                } else {
                                    todo!()
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
                    let filename;
                    if let Ok(vec) = read_vec(&stream) {
                        filename = vec;
                    } else {
                        todo!()
                    }
                    let filename = OsString::from_vec(filename);
                    //??
                    todo!()
                }
                RenameFile => {
                    let old_filename;
                    if let Ok(vec) = read_vec(&stream) {
                        old_filename = vec;
                    } else {
                        todo!()
                    }
                    let old_filename = OsString::from_vec(old_filename);

                    let new_filename;
                    if let Ok(vec) = read_vec(&stream) {
                        new_filename = vec;
                    } else {
                        todo!()
                    }
                    let new_filename = OsString::from_vec(new_filename);

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

                            let filename;
                            if let Ok(vec) = read_vec(&stream) {
                                filename = vec;
                            } else {
                                todo!()
                            }
                            let filename = OsString::from_vec(filename);
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
    }
}
*/
