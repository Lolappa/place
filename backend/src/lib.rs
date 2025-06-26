use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write as _},
    path::Path,
    sync::Mutex,
    time::{Duration, SystemTime, SystemTimeError},
};

use crc::Crc;
use place_constants::{BACKEND_LOCATION, DATA_LOCATION, INTERVAL, SIZE_X, SIZE_Y};
use place_lib::packet::CRC_ALG;
use users::uid_t;

pub mod actions;

pub fn can_do_change(
    userid: uid_t,
    timestamps: &Mutex<HashMap<uid_t, SystemTime>>,
) -> Result<bool, SystemTimeError> {
    if let Ok(timestamps) = timestamps.lock() {
        if let Some(timestamp) = timestamps.get(&userid) {
            match timestamp.elapsed() {
                Ok(time) => Ok(time > Duration::from_secs(INTERVAL)),
                Err(error) => Err(error),
            }
        } else {
            Ok(true)
        }
    } else {
        todo!()
    }
}

pub async fn set_timestamp(
    userid: uid_t,
    timestamps: &Mutex<HashMap<uid_t, SystemTime>>,
) -> io::Result<()> {
    if let Ok(mut timestamps) = timestamps.lock() {
        timestamps.insert(userid, SystemTime::now());
    } else {
        todo!()
    }
    dbg!(timestamps);

    fs::write(
        Path::new(BACKEND_LOCATION).join("user_timestamps"),
        &postcard::to_stdvec_crc32(timestamps, Crc::<u32>::new(&CRC_ALG).digest()).unwrap(),
    )?;

    Ok(())
}

pub fn create_data(force: bool) -> io::Result<()> {
    let data_file = Path::new(DATA_LOCATION).join("data");

    if data_file.exists() == false || force {
        let mut data_file = File::create(data_file)?;
        data_file.write_all(&[0u8; SIZE_X * SIZE_Y])?;
    }

    Ok(())
}
