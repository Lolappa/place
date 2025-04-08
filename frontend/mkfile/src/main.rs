use std::env::args;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, Read, Result, Write};
use std::os::unix::net::UnixStream;

use place_constants::{SOCK_LOCATION, TOKEN};
use place_lib::commands::Command;
use users::get_current_uid;

fn main() {
    let mut args = args().skip(1);
    let x = usize::from_str_radix(
        &args
            .next()
            .expect("Usage: {} <start x> <start y> <width> <height> <filename>"),
        10,
    )
    .expect("Invalid x");
    let y = usize::from_str_radix(
        &args
            .next()
            .expect("Usage: {} <start x> <start y> <width> <height> <filename>"),
        10,
    )
    .expect("Invalid y");
    let dx = usize::from_str_radix(
        &args
            .next()
            .expect("Usage: {} <start x> <start y> <width> <height> <filename>"),
        10,
    )
    .expect("Invalid width");
    let dy = usize::from_str_radix(
        &args
            .next()
            .expect("Usage: {} <start x> <start y> <width> <height> <filename>"),
        10,
    )
    .expect("Invalid height");
    let filename = OsString::from(
        args.next()
            .expect("Usage: {} <start x> <start y> <width> <height> <filename>"),
    );

    if let Err(err) = mkfile((x, y), (dx, dy), &filename) {
        todo!()
    }
}

fn mkfile((x, y): (usize, usize), (dx, dy): (usize, usize), name: &OsStr) -> Result<()> {
    let mut stream = UnixStream::connect(SOCK_LOCATION)?;

    stream.write_all(TOKEN.as_bytes())?;
    stream.write_all(&get_current_uid().to_ne_bytes())?;
    stream.write_all(&[Command::CreateFile as u8])?;

    let mut buf = [0u8];
    stream.read(&mut buf)?;
    if buf[0] == 0 {
        stream.write_all(&x.to_ne_bytes())?;
        stream.write_all(&y.to_ne_bytes())?;
        stream.write_all(&dx.to_ne_bytes())?;
        stream.write_all(&dy.to_ne_bytes())?;
        if buf[0] == 0 {
            todo!()
        } else {
            todo!()
        }
    } else {
        todo!()
    }

    Ok(())
}
