use std::{
    ffi::{OsStr, OsString},
    fs, io,
    os::unix,
    path::Path,
};

use place_constants::{DATA_LOCATION, DIR_LOCATION, FILESYSTEM_LOCATION};
use place_lib::fs::{Directory, File};

pub fn update_file_add(name: &OsStr) -> io::Result<()> {
    let file = File::from_file(Path::new(DATA_LOCATION).join("file").join(name))?;
    let dirs: Vec<(Directory, OsString)> = match fs::read_dir(Path::new(DATA_LOCATION).join("dir"))
    {
        Ok(value) => {
            let mut tmp = Vec::<(Directory, OsString)>::new();
            for entry in value {
                if let Ok(value) = entry {
                    tmp.push((Directory::from_file(value.path())?, value.file_name()));
                }
            }
            tmp
        }
        Err(_) => todo!(),
    };

    let containers = dirs.into_iter().filter(|x| file.is_inside(x.0));

    let containers = containers
        .clone()
        .filter(|x| {
            !containers
                .clone()
                .filter(|y| y.0 != x.0)
                .any(|y| x.0.is_inside(y.0))
        })
        .collect::<Vec<(Directory, OsString)>>();

    if containers.len() == 0 {
        fs::hard_link(
            Path::new(DATA_LOCATION).join("file").join(name),
            Path::new(FILESYSTEM_LOCATION).join(name),
        )?;
    } else {
        for (_, dir) in containers {
            fs::hard_link(
                Path::new(DATA_LOCATION).join("file").join(name),
                Path::new(DIR_LOCATION).join(dir).join(name),
            )?;
        }
    }

    Ok(())
}

pub fn update_dir_add(name: &OsStr) -> io::Result<()> {
    let dir = Directory::from_file(Path::new(DATA_LOCATION).join("dir").join(name))?;
    let dirs: Vec<(Directory, OsString)> = match fs::read_dir(Path::new(DATA_LOCATION).join("dir"))
    {
        Ok(value) => {
            let mut tmp = Vec::<(Directory, OsString)>::new();
            for entry in value {
                if let Ok(value) = entry {
                    tmp.push((Directory::from_file(value.path())?, value.file_name()));
                }
            }
            tmp
        }
        Err(_) => todo!(),
    };

    let containers = dirs.into_iter().filter(|x| dir.is_inside(x.0));

    let containers = containers
        .clone()
        .filter(|x| {
            !containers
                .clone()
                .filter(|y| y.0 != x.0)
                .any(|y| x.0.is_inside(y.0))
        })
        .collect::<Vec<(Directory, OsString)>>();

    let dir_path = Path::new(DIR_LOCATION).join(name);

    fs::create_dir(&dir_path)?;
    if containers.len() != 0 {
        for (_, dir) in containers {
            unix::fs::symlink(&dir_path, Path::new(DIR_LOCATION).join(dir).join(name))?;
        }
    }

    Ok(())
}
