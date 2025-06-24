use std::{
    env::{self, args},
    usize,
};

use place_lib::{commands::Command, packet::HeaderBlock, syscalls};

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

    let header = HeaderBlock::new(uid, Command::SetByte);
}
