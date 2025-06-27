use std::{env, usize};

use place_lib::{
    commands::Command,
    fs::Position,
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

    let packet = Packet::new(vec![header, content]);

    place_internal_front_lib::send_packet(packet);
}
