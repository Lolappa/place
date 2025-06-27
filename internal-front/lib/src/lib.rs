use std::{
    io::{Read as _, Write as _},
    os::unix::net::UnixStream,
};

use chacha20poly1305::{aead::Aead as _, KeyInit, XNonce};
use place_constants::{Crypt, CRYPT_KEY, SOCK_LOCATION};
use place_lib::packet::Packet;

pub fn send_packet(packet: Packet) {
    // Serialize packet
    let packet = match packet.to_stdvec() {
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
    let crypt = Crypt::new(CRYPT_KEY.into());
    let mut nonce = XNonce::default();
    if let Err(error) = stream.read_exact(&mut nonce) {
        eprintln!("Failed to read from server: {}", error);
        return;
    }

    // Encrypt packet
    let ciphertext = match crypt.encrypt(&nonce, packet.as_slice()) {
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
