use place_constants::{ SIZE_X, SIZE_Y, LOCATION };
use std::path::{ Path, PathBuf };
use std::fs;

struct Options {
    files: Vec<PathBuf>,
}

impl Options {
    fn new() -> Options {
        Options {
            files: vec![],
        }
    }
}

fn main() {
    if std::env::args().len() > 1 {
        let mut args = std::env::args().skip(1);
        let mut options = Options::new();
        while let Some(argument) = args.next() {
            match argument.as_str() {
                str if str.starts_with("--") => {
                    todo!();
                    let str = str.split_at(2).1;
                }
                str if str.starts_with("-") => {
                    let str = str.split_at(1).1;
                    for char in str.as_bytes().iter() {
                        match char {
                            argument => {
                                eprintln!("Unknown argument: -{}", *argument as char);
                            }
                        }
                    }
                }
                str => {
                    options.files.push(PathBuf::from(str));
                }
            }
        }
        file(options);
    } else {
        print((0, 0), (SIZE_X, SIZE_Y));
    }
}

fn file(options: Options) {
    for file in &options.files {
    if let Ok(content) = fs::read(file) {
        let x = u64::from_le_bytes(content[0..8].try_into().expect("Invalid file"));
        let y = u64::from_le_bytes(content[8..16].try_into().expect("Invalid file"));
        let dx = u64::from_le_bytes(content[16..24].try_into().expect("Invalid file"));
        let dy = u64::from_le_bytes(content[24..32].try_into().expect("Invalid file"));
        println!("{}:", file.display());
        print((x, y), (dx.try_into().unwrap(), dy.try_into().unwrap()));
    } else {
        eprintln!("{}: No such file or directory", &file.display());
        continue;
    }
    }
}

fn print(start: (u64, u64), size: (usize, usize)) {
    if size.0 == 0 || size.1 == 0 {
        println!("Empty file");
        return;
    }
    let content = fs::read(Path::new(LOCATION).join("data/data")).unwrap();

    for y in start.1 as usize..start.1 as usize + size.1 {
        print!("{:02x}", content[y * SIZE_X]);
        for x in (start.0 + 1) as usize..start.0 as usize + size.0 {
            print!(" {:02x}", content[y * SIZE_X + x]);
        }
        println!();
    }
}
