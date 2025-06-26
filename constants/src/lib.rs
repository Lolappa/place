use chacha20poly1305::XChaCha20Poly1305;

pub type Crypt = XChaCha20Poly1305;

macro_rules! out_dir {
    ($x:expr) => {
        concat!(env!("OUT_DIR"), $x)
    };
}

pub const INTERVAL: u64 = include!(out_dir!("/interval"));
pub const SIZE_X: usize = include!(out_dir!("/size_x"));
pub const SIZE_Y: usize = include!(out_dir!("/size_y"));
pub const LOCATION: &'static str = include!(out_dir!("/location"));
pub const SOCK_LOCATION: &'static str = concat!(include!(out_dir!("/location")), "/.sock",);
pub const TIMESTAMP_LOCATION: &'static str =
    concat!(include!(out_dir!("/location")), "/backend/user_timestamps",);
pub const CRYPT_KEY: &'static [u8; 32] = include_bytes!(out_dir!("/crypt_key"));
