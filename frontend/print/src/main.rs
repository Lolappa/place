use place_constants::{ SIZE_X, SIZE_Y, LOCATION };
use std::path::Path;
use std::fs;

fn main() {
    if let Some(file) = std::env::args().nth(1) {
        todo!();
        print(&Path::new(&file));
    } else {
        print(&Path::new(LOCATION).join("data/data"));
    }
}

fn print(file: &Path) {
    let content = if let Ok(vec) = fs::read(file) {
        vec
    } else {
        eprintln!("{}: No such file or directory", file.display());
        return;
    };

    for y in 0..SIZE_Y {
        print!("{:02x}", content[y * SIZE_X]);
        for x in 1..SIZE_X {
            print!(" {:02x}", content[y * SIZE_X + x]);
        }
        println!();
    }
}
