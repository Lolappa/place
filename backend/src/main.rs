use place_constants::{ SIZE_X, SIZE_Y };
use std::io::Write;
use std::path::Path;
use std::fs::File;

fn main() {
    println!("{}", env!("INTERVAL"));
}

fn create_data() {
    let data_file = Path::new(
        &std::env::args()
            .next()
            .unwrap())
        .parent().unwrap()
        .parent().unwrap()
        .join("/data/data");

    if data_file.exists() != true {
        let mut data_file = File::create(data_file).unwrap();
        data_file.write_all(&[0u8; SIZE_X * SIZE_Y]);
    }
}
