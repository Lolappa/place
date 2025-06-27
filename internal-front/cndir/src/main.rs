use std::{env, ffi::OsString};

use place_lib::{
    commands::Command,
    packet::{Block, Packet},
    syscalls,
};

fn main() {
    let mut args = env::args().skip(1);

    let from: OsString = if let Some(value) = args.next() {
        value.into()
    } else {
        usage_error();
        return;
    };

    let to: OsString = if let Some(value) = args.next() {
        value.into()
    } else {
        usage_error();
        return;
    };

    let uid = syscalls::get_current_uid();

    let header = Block::HeaderBlock {
        uid,
        command: Command::RenameDir,
    };

    let content_1 = Block::ObjectName(from);
    let content_2 = Block::ObjectName(to);

    let packet = Packet::new(vec![header, content_1, content_2]);

    place_internal_front_lib::send_packet(packet);
}

fn usage_error() {
    eprintln!("Usage: {} <from> <to>", env::args().nth(0).unwrap());
}
