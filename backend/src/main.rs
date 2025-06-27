use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, KeyInit,
};
use place_backend::*;
use place_constants::*;
use place_lib::{
    commands::Command,
    fs::{File, Position, Size},
    packet::{Block, Packet},
};
use std::{
    collections::HashMap,
    ffi::OsString,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};
use users::uid_t;

fn main() {
    // NOTE: debug actions
    let _ = create_data(false);
    let _ = actions::write_byte(Position::new(3, 3), 50);
    let _ = actions::create_file(
        File::new(Position::new(1, 1), Size::new(5, 5)),
        &OsString::from("tst"),
    );
    let _ = actions::create_file(
        File::new(Position::new(1, 1), Size::new(0, 0)),
        &OsString::from("empty"),
    );

    let timestamps = Arc::new(Mutex::new(HashMap::<uid_t, SystemTime>::new()));
    match UnixListener::bind(SOCK_LOCATION) {
        Ok(listener) => {
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
        Err(error) => eprintln!("Failed to bind to socket: {}", error),
    }
    println!("exiting");
}

fn handle_client(mut stream: UnixStream, timestamps: &Mutex<HashMap<uid_t, SystemTime>>) {
    let crypt = Crypt::new(CRYPT_KEY.into());
    let nonce = Crypt::generate_nonce(OsRng);

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

    let (uid, command) = if let Block::HeaderBlock { uid, command } = packet.blocks()[0] {
        (uid, command)
    } else {
        panic!()
    };

    let can_change = match can_do_change(uid, timestamps) {
        Ok(value) => value,
        Err(err) => todo!(),
    };
    dbg!(can_change);
    if can_change {
        match command {
            Command::SetByte => {
                if let Some(Block::SetByteContent { position, value }) = packet.blocks().get(1) {
                    if let Err(err) = actions::write_byte(*position, *value) {
                        todo!()
                    };
                } else {
                    todo!()
                }
            }
            // TODO: Make the following actions update the filesystem representation
            Command::CreateFile => {
                let file = match packet.blocks().get(1) {
                    Some(Block::ObjectSize(value)) => value,
                    _ => todo!(),
                };
                let name = match packet.blocks().get(2) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::create_file(*file, name) {
                    todo!();
                };
            }
            Command::MoveFile => {
                let name = match packet.blocks().get(1) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };
                let file = match packet.blocks().get(2) {
                    Some(Block::ObjectSize(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::move_file(name, *file) {
                    todo!();
                };
            }
            Command::RemoveFile => {
                let name = match packet.blocks().get(1) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::remove_file(name) {
                    todo!();
                };
            }
            Command::RenameFile => {
                let from = match packet.blocks().get(1) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };
                let to = match packet.blocks().get(2) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::rename_file(&from, &to) {
                    todo!();
                };
            }
            Command::CreateDir => {
                let dir = match packet.blocks().get(1) {
                    Some(Block::ObjectSize(value)) => value,
                    _ => todo!(),
                };
                let name = match packet.blocks().get(2) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::create_dir(*dir, name) {
                    todo!();
                };
            }
            Command::MoveDir => {
                let name = match packet.blocks().get(2) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };
                let dir = match packet.blocks().get(1) {
                    Some(Block::ObjectSize(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::move_dir(name, *dir) {
                    todo!();
                };
            }
            Command::RemoveDir => {
                let name = match packet.blocks().get(1) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::remove_dir(name) {
                    todo!();
                };
            }
            Command::RenameDir => {
                let from = match packet.blocks().get(1) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };
                let to = match packet.blocks().get(2) {
                    Some(Block::ObjectName(value)) => value,
                    _ => todo!(),
                };

                if let Err(err) = actions::rename_dir(&from, &to) {
                    todo!();
                };
            }
        }
        set_timestamp(uid, timestamps);
    }
}
