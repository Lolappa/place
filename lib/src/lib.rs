#[allow(unused)]
pub mod syscalls;

pub mod commands {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Copy)]
    pub enum Command {
        SetByte = 1,
        CreateFile = 2,
        RemoveFile = 3,
        RenameFile = 4,
        MoveFile = 5,
    }

    impl TryFrom<u8> for Command {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            use Command::*;
            match value {
                _ if value == SetByte as u8 => Ok(SetByte),
                _ if value == CreateFile as u8 => Ok(CreateFile),
                _ if value == RemoveFile as u8 => Ok(RemoveFile),
                _ if value == RenameFile as u8 => Ok(RenameFile),
                _ if value == MoveFile as u8 => Ok(MoveFile),
                _ => Err(()),
            }
        }
    }
}

pub mod packet {
    use std::ffi::OsString;

    use crc::{Algorithm, Crc, Digest};
    use postcard::Result;
    use serde::{Deserialize, Serialize};
    use users::uid_t;

    use crate::commands::Command;

    pub const CRC_ALG: Algorithm<u32> = crc::CRC_24_OPENPGP;

    #[derive(Clone)]
    pub struct Packet {
        blocks: Vec<Block>,
    }

    impl Packet {
        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn from_bytes<'a>(s: &'a [u8]) -> Result<Packet> {
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

            Ok(Packet { blocks })
        }

        pub fn to_stdvec(&self) -> Result<Vec<u8>> {
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
        SetByteContent { x: usize, y: usize, value: u8 },
        OsString(OsString),
    }

    impl Block {
        pub fn take_from_bytes<'a>(
            s: &'a [u8],
            digest: Digest<'a, u32>,
        ) -> Result<(Self, &'a [u8])> {
            postcard::take_from_bytes_crc32(s, digest)
        }

        pub fn to_stdvec(&self) -> Result<Vec<u8>> {
            postcard::to_stdvec_crc32(&self, Crc::<u32>::new(&CRC_ALG).digest())
        }
    }
}
