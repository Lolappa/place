use std::{env, ffi::OsString};

use place_lib::{
    commands::Command,
    fs::{Directory, Position},
    packet::{Block, Packet},
    syscalls,
};

fn main() {
    let mut args = env::args().skip(1);

    let name: OsString = if let Some(value) = args.next() {
        value.into()
    } else {
        usage_error();
        return;
    };

    let start_x: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid start x: {}", error);
                return;
            }
        }
    } else {
        usage_error();
        return;
    };

    let start_y: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid start y: {}", error);
                return;
            }
        }
    } else {
        usage_error();
        return;
    };

    let end_x: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid end x: {}", error);
                return;
            }
        }
    } else {
        usage_error();
        return;
    };

    let end_y: usize = if let Some(value) = args.next() {
        match usize::from_str_radix(&value, 10) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Invalid end y: {}", error);
                return;
            }
        }
    } else {
        usage_error();
        return;
    };

    let start_pos = Position::new(start_x, start_y);
    let end_pos = Position::new(end_x, end_y);

    let dir = Directory::from_start_end(start_pos, end_pos);

    let uid = syscalls::get_current_uid();

    let header = Block::HeaderBlock {
        uid,
        command: Command::CreateDir,
    };

    let content_1 = Block::ObjectSize(dir);
    let content_2 = Block::ObjectName(name);

    let packet = Packet::new(vec![header, content_1, content_2]);

    place_internal_front_lib::send_packet(packet);
}

fn usage_error() {
    eprintln!(
        "Usage: {} <name> <start x> <start y> <end x> <end y>",
        env::args().nth(0).unwrap()
    );
}
