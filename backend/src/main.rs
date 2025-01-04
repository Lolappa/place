use place_constants::*;
use std::io::prelude::*;
use std::io::SeekFrom::Start;
use std::path::Path;
use std::fs::File;

fn main() {
    create_data();
    write_byte(4, 31, 50);
    create_file((2, 2), (4, 4), "tst");
    create_file((2, 2), (0, 0), "empty");
}

fn create_data() {
    let data_file = Path::new(LOCATION).join("data/data");

    if data_file.exists() != true {
        let mut data_file = File::create(data_file).unwrap();
        let _ = data_file.write_all(&[0u8; SIZE_X * SIZE_Y]);
    }
}

fn write_byte(x: usize, y: usize, value: u8) {
    if x >= SIZE_X || y >= SIZE_Y {
        return;
    }
    let mut data_file = File::options()
        .write(true)
        .open(Path::new(LOCATION).join("data/data"))
        .unwrap();
    let _ = data_file.seek(Start((y * SIZE_X + x).try_into().unwrap()));
    let _ = data_file.write(&[value]);
}

fn create_file(pos: (u64, u64), size: (u64, u64), name: &str) {
    let mut file = if let Ok(file) = File::create_new(Path::new(LOCATION).join("data/files").join(name)) {
        file
    } else {
        return;
    };
    let _ = file.write(&pos.0.to_le_bytes());
    let _ = file.write(&pos.1.to_le_bytes());
    let _ = file.write(&size.0.to_le_bytes());
    let _ = file.write(&size.1.to_le_bytes());
}
