use place_constants::*;
use std::io::Write;
use std::path::Path;
use std::fs::File;

fn main() {
    create_data();
}

fn create_data() {
    let data_file = Path::new(LOCATION).join("data/data");

    if data_file.exists() != true {
        let mut data_file = File::create(data_file).unwrap();
        data_file.write_all(&[0u8; SIZE_X * SIZE_Y]);
    }
}
