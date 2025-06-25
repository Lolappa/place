use std::{
    env,
    io::{Read, Write},
    os::unix::net::UnixStream,
    usize,
};

use chacha20poly1305::{aead::Aead, KeyInit, XChaCha20Poly1305, XNonce};
use place_constants::{CRYPT_KEY, SOCK_LOCATION};
use place_lib::{
    commands::Command,
    file::Position,
    packet::{Block, Packet},
    syscalls,
};

fn main() {
    let mut args = env::args().skip(1);

    let x: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid x: {}", error);
                return;
            }
        }
    } else {
        eprintln!("Usage: {} <x> <y> <value>", env::args().nth(0).unwrap());
        return;
    };

    let y: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid y: {}", error);
                return;
            }
        }
    } else {
        eprintln!("Usage: {} <x> <y> <value>", env::args().nth(0).unwrap());
        return;
    };

    let position = Position::new(x, y);

    let value: u8 = if let Some(value) = args.next() {
        match u8::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid y: {}", error);
                return;
            }
        }
    } else {
        eprintln!("Usage: {} <x> <y> <value>", env::args().nth(0).unwrap());
        return;
    };

    let uid = syscalls::get_current_uid();

    let header = Block::HeaderBlock {
        uid,
        command: Command::SetByte,
    };

    let content = Block::SetByteContent { position, value };

    let packet = match Packet::new(vec![header, content]).to_stdvec() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to serialize packet: {}", error);
            return;
        }
    };

    // Connect to backend
    let mut stream = match UnixStream::connect(SOCK_LOCATION) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to connect: {}", error);
            return;
        }
    };

    // Initialize crypt
    let crypt = XChaCha20Poly1305::new(CRYPT_KEY.into());
    let mut nonce = XNonce::default();
    if let Err(error) = stream.read_exact(&mut nonce) {
        eprintln!("Failed to read from server: {}", error);
        return;
    }

    // Encrypt packet
    let ciphertext = match crypt.encrypt(&nonce, &*packet) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to encrypt packet: {}", error);
            return;
        }
    };

    // Send packet
    if let Err(error) = stream.write_all(&ciphertext.len().to_ne_bytes()) {
        eprintln!("Failed to write to server: {}", error);
        return;
    }
    if let Err(error) = stream.write_all(&ciphertext) {
        eprintln!("Failed to write to server: {}", error);
        return;
    }
}
