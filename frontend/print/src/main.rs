use place_constants::{ SIZE_X, SIZE_Y, LOCATION };
use std::path::Path;
use std::fs;

fn main() {
    /*
    let args = std::env::args().skip(1);
    for argument in args {
        match argument.as_str() {
            "-i"
        }
    }
    */
    if let Some(file) = std::env::args().nth(1) {
        todo!();
    } else {
        print((0, 0), (SIZE_X, SIZE_Y));
    }
}

fn file(file: &Path) {
    
        eprintln!("{}: No such file or directory", file.display());
        return;
}

fn print(start: (u64, u64), size: (usize, usize)) {
    let content = fs::read(Path::new(LOCATION).join("data/data")).unwrap();

    for y in start.1 as usize..start.1 as usize + SIZE_Y {
        print!("{:02x}", content[y * SIZE_X]);
        for x in (start.0 + 1) as usize..start.0 as usize + SIZE_X {
            print!(" {:02x}", content[y * SIZE_X + x]);
        }
        println!();
    }
}
