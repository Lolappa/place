pub mod fs;
pub mod syscalls;

pub mod commands {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Copy)]
    pub enum Command {
        SetByte = 1,
        CreateFile = 2,
        MoveFile = 3,
        RemoveFile = 4,
        RenameFile = 5,
        CreateDir = 6,
        MoveDir = 7,
        RemoveDir = 8,
        RenameDir = 9,
    }

    impl TryFrom<u8> for Command {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            use Command::*;
            match value {
                _ if value == SetByte as u8 => Ok(SetByte),
                _ if value == CreateFile as u8 => Ok(CreateFile),
                _ if value == MoveFile as u8 => Ok(MoveFile),
                _ if value == RemoveFile as u8 => Ok(RemoveFile),
                _ if value == RenameFile as u8 => Ok(RenameFile),
                _ if value == CreateDir as u8 => Ok(CreateDir),
                _ if value == MoveDir as u8 => Ok(MoveDir),
                _ if value == RemoveDir as u8 => Ok(RemoveDir),
                _ if value == RenameDir as u8 => Ok(RenameDir),
                _ => Err(()),
            }
        }
    }
}

pub mod packet {
    use std::ffi::OsString;

    use crc::{Algorithm, Crc, Digest};
    use postcard::{Error, Result as PostcardResult};
    use serde::{Deserialize, Serialize};
    use users::uid_t;

    use crate::{
        commands::Command,
        fs::{File, PlaceObject, Position},
    };

    pub const CRC_ALG: Algorithm<u32> = crc::CRC_24_OPENPGP;

    #[derive(Clone)]
    pub struct Packet {
        blocks: Vec<Block>,
    }

    impl Packet {
        pub fn new(blocks: Vec<Block>) -> Self {
            Self { blocks }
        }

        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn from_bytes<'a>(s: &'a [u8]) -> PostcardResult<Packet> {
            let mut blocks: Vec<Block> = Vec::new();
            let mut tail = s;
            let crc = Crc::<u32>::new(&CRC_ALG);
            loop {
                let block: Block;
                (block, tail) = Block::take_from_bytes(tail, crc.digest())?;

                blocks.push(block);
                if tail.len() == 0 {
                    break;
                }
            }

            if blocks.len() == 0 {
                return Err(Error::SerdeDeCustom);
            }
            if !blocks[0].is_header_block() {
                return Err(Error::SerdeDeCustom);
            }

            Ok(Packet { blocks })
        }

        pub fn to_stdvec(&self) -> PostcardResult<Vec<u8>> {
            let mut out = Vec::<u8>::new();
            for block in &self.blocks {
                out.extend_from_slice(&block.to_stdvec()?);
            }
            Ok(out)
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum Block {
        HeaderBlock { uid: uid_t, command: Command },
        SetByteContent { position: Position, value: u8 },
        ObjectSize(PlaceObject),
        ObjectName(OsString),
    }

    impl Block {
        pub fn take_from_bytes<'a>(
            s: &'a [u8],
            digest: Digest<'a, u32>,
        ) -> PostcardResult<(Self, &'a [u8])> {
            postcard::take_from_bytes_crc32(s, digest)
        }

        pub fn to_stdvec(&self) -> PostcardResult<Vec<u8>> {
            postcard::to_stdvec_crc32(&self, Crc::<u32>::new(&CRC_ALG).digest())
        }

        /// Returns `true` if the block is [`HeaderBlock`].
        ///
        /// [`HeaderBlock`]: Block::HeaderBlock
        #[must_use]
        pub fn is_header_block(&self) -> bool {
            matches!(self, Self::HeaderBlock { .. })
        }

        /// Returns `true` if the block is [`SetByteContent`].
        ///
        /// [`SetByteContent`]: Block::SetByteContent
        #[must_use]
        pub fn is_set_byte_content(&self) -> bool {
            matches!(self, Self::SetByteContent { .. })
        }

        /// Returns `true` if the block is [`FileSize`].
        ///
        /// [`FileSize`]: Block::FileSize
        #[must_use]
        pub fn is_file_size(&self) -> bool {
            matches!(self, Self::ObjectSize(..))
        }

        pub fn as_file_size(&self) -> Option<&File> {
            if let Self::ObjectSize(v) = self {
                Some(v)
            } else {
                None
            }
        }

        pub fn try_into_file_size(self) -> Result<File, Self> {
            if let Self::ObjectSize(v) = self {
                Ok(v)
            } else {
                Err(self)
            }
        }

        /// Returns `true` if the block is [`FileName`].
        ///
        /// [`FileName`]: Block::FileName
        #[must_use]
        pub fn is_file_name(&self) -> bool {
            matches!(self, Self::ObjectName(..))
        }

        pub fn as_file_name(&self) -> Option<&OsString> {
            if let Self::ObjectName(v) = self {
                Some(v)
            } else {
                None
            }
        }

        pub fn try_into_file_name(self) -> Result<OsString, Self> {
            if let Self::ObjectName(v) = self {
                Ok(v)
            } else {
                Err(self)
            }
        }
    }
}
