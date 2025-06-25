use std::{env, fs, path::Path};

use chacha20poly1305::{aead::OsRng, KeyInit, XChaCha20Poly1305};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    location: String,
    width: usize,
    height: usize,
    interval: u64,
}

type Crypt = XChaCha20Poly1305;

macro_rules! out_dir {
    () => {
        env::var("OUT_DIR").unwrap()
    };
}

fn main() {
    println!("cargo::rerun-if-changed=../config/");

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
            if let Err(error) =
                fs::write(out_dir!() + "/interval", format!("{:#X}", config.interval))
            {
                println!("cargo::error=Failed to write: {}", error);
            }

            if let Err(error) = fs::write(out_dir!() + "/size_x", format!("{:#x}", config.width)) {
                println!("cargo::error=Failed to write: {}", error);
            }

            if let Err(error) = fs::write(out_dir!() + "/size_y", format!("{:#x}", config.height)) {
                println!("cargo::error=Failed to write: {}", error);
            }

            // NOTE: might redo this one
            if let Err(error) = fs::write(
                out_dir!() + "/location",
                format!("{:?}", Path::new(&config.location)),
            ) {
                println!("cargo::error=Failed to write: {}", error);
            }

            if let Err(error) = fs::write(
                out_dir!() + "/crypt_key",
                Crypt::generate_key(OsRng).to_vec(), // binary file
            ) {
                println!("cargo::error=Failed to write: {}", error);
            }

            /*echo "pub const INTERVAL: usize = ${interval};" > constants/constants.rs
            #echo "pub const TOKEN: &str = \"${token}\";" >> constants/constants.rs
            echo "pub const SIZE_X: usize = ${size_x};" >> constants/constants.rs
            echo "pub const SIZE_Y: usize = ${size_y};" >> constants/constants.rs
            echo "pub const LOCATION: &str = \"${location}\";" >> constants/constants.rs
            echo "pub const SOCK_LOCATION: &str = \"${location}.sock\";" >> constants/constants.rs*/
        }
        Err(error) => println!("cargo::error=Invalid config: {}", error),
    };
}
