use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write as _},
    path::Path,
    sync::Mutex,
    time::{Duration, SystemTime, SystemTimeError},
};

use place_constants::{INTERVAL, LOCATION, SIZE_X, SIZE_Y};
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

pub fn set_timestamp(userid: uid_t, timestamps: &Mutex<HashMap<uid_t, SystemTime>>) {
    if let Ok(mut timestamps) = timestamps.lock() {
        timestamps.insert(userid, SystemTime::now());
    } else {
        todo!()
    }
    dbg!(timestamps);
    // TODO: Save hashmap in case the server crashes
}

pub fn create_data(force: bool) -> io::Result<()> {
    let data_file = Path::new(LOCATION).join("data/data");

    if data_file.exists() == false || force {
        let mut data_file = File::create(data_file)?;
        data_file.write_all(&[0u8; SIZE_X * SIZE_Y])?;
    }

    Ok(())
}
