use std::{array::TryFromSliceError, fs, io, ops::Add, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pair {
    x: usize,
    y: usize,
}

pub type Position = Pair;
pub type Size = Pair;

impl Pair {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlaceObject {
    position: Position,
    size: Size,
}

pub type File = PlaceObject;
pub type Directory = PlaceObject;

impl PlaceObject {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    pub fn start_pos(&self) -> Position {
        self.position
    }

    pub fn end_pos(&self) -> Position {
        self.position + self.size
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn to_stdvec(&self) -> Vec<u8> {
        let mut out = Vec::<u8>::new();

        out.extend_from_slice(&self.start_pos().x.to_ne_bytes());
        out.extend_from_slice(&self.start_pos().y.to_ne_bytes());
        out.extend_from_slice(&self.end_pos().x.to_ne_bytes());
        out.extend_from_slice(&self.end_pos().y.to_ne_bytes());

        out
    }

    pub fn from_bytes<'a>(s: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let mut slice = s;
        let x = {
            let tmp: &'a [u8];
            (tmp, slice) = slice.split_at(size_of::<usize>());
            usize::from_ne_bytes(tmp.try_into()?)
        };
        let y = {
            let tmp: &'a [u8];
            (tmp, slice) = slice.split_at(size_of::<usize>());
            usize::from_ne_bytes(tmp.try_into()?)
        };
        let dx = {
            let tmp: &'a [u8];
            (tmp, slice) = slice.split_at(size_of::<usize>());
            usize::from_ne_bytes(tmp.try_into()?)
        };
        let dy = usize::from_ne_bytes(slice.try_into()?);

        let position = Position::new(x, y);
        let size = Size::new(dx, dy);
        Ok(Self { position, size })
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> io::Result<Self> {
        match Self::from_bytes(&fs::read(path)?) {
            Ok(value) => Ok(value),
            Err(error) => Err(io::Error::new(io::ErrorKind::UnexpectedEof, error)),
        }
    }
}
