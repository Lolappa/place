use std::{env, ffi::OsString};

use place_lib::{
    commands::Command,
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

    let uid = syscalls::get_current_uid();

    let header = Block::HeaderBlock {
        uid,
        command: Command::RemoveDir,
    };

    let content = Block::ObjectName(name);

    let packet = Packet::new(vec![header, content]);

    place_internal_front_lib::send_packet(packet);
}

fn usage_error() {
    eprintln!("Usage: {} <name>", env::args().nth(0).unwrap());
}
