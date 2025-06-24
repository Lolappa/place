use std::{env, fmt::Write, fs};

use chacha20poly1305::{aead::OsRng, KeyInit, XChaCha20Poly1305};
use serde::{de::value, Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    location: String,
    width: usize,
    height: usize,
    interval: u64,
}

fn main() {
    let mut config = String::new();
    for entry in fs::read_dir("../config/").expect("tuuba") {
        let entry = if let Ok(value) = entry {
            value
        } else {
            break;
        };
        if entry.file_type().unwrap().is_file() {
            config.push_str(&fs::read_to_string(entry.path()).unwrap());
        }
    }
    match toml::from_str::<Config>(&config) {
        Ok(config) => {
            let mut content = String::new();

            writeln!(&mut content, "const SIZE_X: usize = {:?};", config.width).unwrap();
            writeln!(&mut content, "const SIZE_Y: usize = {:?};", config.height).unwrap();
            writeln!(&mut content, "const INTERVAL: u64 = {:?};", config.interval).unwrap();
            writeln!(
                &mut content,
                "const LOCATION: &str = {:?};",
                config.location
            )
            .unwrap();
            writeln!(
                &mut content,
                "const SOCK_LOCATION: &str = {:?};",
                config.location + ".sock"
            )
            .unwrap();
            writeln!(
                &mut content,
                "const CRYPT_KEY: [u8; 32] = {:?};",
                XChaCha20Poly1305::generate_key(OsRng)
            )
            .unwrap();
            /*echo "pub const INTERVAL: usize = ${interval};" > constants/constants.rs
            #echo "pub const TOKEN: &str = \"${token}\";" >> constants/constants.rs
            echo "pub const SIZE_X: usize = ${size_x};" >> constants/constants.rs
            echo "pub const SIZE_Y: usize = ${size_y};" >> constants/constants.rs
            echo "pub const LOCATION: &str = \"${location}\";" >> constants/constants.rs
            echo "pub const SOCK_LOCATION: &str = \"${location}.sock\";" >> constants/constants.rs*/

            if let Err(error) = fs::write(env::var("OUT_DIR").unwrap() + "/constants.rs", content) {
                println!("cargo::error=Failed to create constant file: {}", error)
            };
        }
        Err(error) => println!("cargo::error=Invalid config: {}", error),
    };
}
