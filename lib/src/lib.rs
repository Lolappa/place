#[allow(unused)]
pub mod syscalls;

pub mod commands {
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
            match value {
                1 => Ok(Self::SetByte),
                2 => Ok(Self::CreateFile),
                3 => Ok(Self::RemoveFile),
                4 => Ok(Self::RenameFile),
                5 => Ok(Self::MoveFile),
                _ => Err(()),
            }
        }
    }
}

pub mod packet {
    use crc::{Algorithm, Crc};
    use users::uid_t;

    use crate::commands::Command;

    pub const CRC_ALG: Algorithm<u32> = crc::CRC_24_OPENPGP;

    pub struct Packet {
        header: HeaderBlock,
        blocks: Vec<Block>,
    }

    impl Packet {
        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn blocks_mut(&mut self) -> &mut Vec<Block> {
            &mut self.blocks
        }

        pub fn header(&self) -> &HeaderBlock {
            &self.header
        }
    }

    impl TryFrom<&[u8]> for Packet {
        type Error = (); // TODO: use an actual type here

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let mut blocks: Vec<Block> = vec![];

            let mut slice = &value[..];

            // Get header block
            let header = match Block::try_from(slice) {
                Ok(value) => match HeaderBlock::try_from(value) {
                    Ok(value) => value,
                    Err(_) => todo!(),
                },
                Err(_) => {
                    todo!()
                }
            };
            slice = &slice[header.len()..];

            // Get the rest of the blocks
            while slice.len() > 0 {
                let block = match Block::try_from(slice) {
                    Ok(value) => value,
                    Err(_) => {
                        todo!()
                    }
                };
                slice = &slice[block.len()..];
                blocks.push(block);
            }

            Ok(Packet { header, blocks })
        }
    }

    impl From<Packet> for Vec<u8> {
        fn from(value: Packet) -> Self {
            let mut out: Self =
                Self::with_capacity(value.blocks.iter().fold(0, |acc, block| acc + block.len()));

            for block in value.blocks {
                out.extend_from_slice(&Vec::from(block));
            }

            out
        }
    }

    const HEADER_LENGTH: usize = size_of::<uid_t>() + 1;
    pub struct HeaderBlock {
        uid: uid_t,
        command: Command,
    }

    impl HeaderBlock {
        pub fn new(uid: uid_t, command: Command) -> Self {
            Self { uid, command }
        }

        pub fn len(&self) -> usize {
            size_of::<usize>() + 1 + HEADER_LENGTH + 1
        }

        pub fn uid(&self) -> u32 {
            self.uid
        }

        pub fn command(&self) -> &Command {
            &self.command
        }
    }

    impl TryFrom<Block> for HeaderBlock {
        type Error = (); // TODO: Use an actual type here

        fn try_from(value: Block) -> Result<Self, Self::Error> {
            if value.len() != HEADER_LENGTH {
                return Err(());
            } else {
                let mut slice = value.content();

                let uid: uid_t = {
                    let tmp: &[u8];
                    (tmp, slice) = slice.split_at(size_of::<uid_t>());
                    uid_t::from_ne_bytes(tmp.try_into().unwrap())
                };

                let command: Command = {
                    let tmp: &[u8];
                    (tmp, slice) = slice.split_at(1);
                    Command::try_from(u8::from_ne_bytes(tmp.try_into().unwrap()))?
                };

                Ok(HeaderBlock { uid, command })
            }
        }
    }

    impl TryFrom<&[u8]> for HeaderBlock {
        type Error = (); // TODO: Use an actual type here

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            Self::try_from(Block::try_from(value)?)
        }
    }

    impl From<HeaderBlock> for Vec<u8> {
        fn from(value: HeaderBlock) -> Self {
            let mut out: Self = Self::with_capacity(value.len());

            out.extend_from_slice(&HEADER_LENGTH.to_ne_bytes());
            out.push(BlockType::Header as u8);
            out.extend_from_slice(&value.uid.to_ne_bytes());
            out.push(value.command as u8);
            out.extend_from_slice(&Crc::<u32>::new(&CRC_ALG).checksum(&out[..]).to_ne_bytes());

            out
        }
    }

    pub struct Block {
        block_type: BlockType,
        content: Vec<u8>,
    }

    impl Block {
        pub fn block_type(&self) -> &BlockType {
            &self.block_type
        }

        pub fn content(&self) -> &[u8] {
            &self.content
        }

        pub fn len(&self) -> usize {
            size_of::<usize>() + 1 + &self.content.len() + 1
        }
    }
    impl TryFrom<&[u8]> for Block {
        type Error = (); // TODO: Use an actual type here

        fn try_from(value: &[u8]) -> Result<Block, Self::Error> {
            let size = match value.get(..size_of::<usize>()) {
                Some(size) => usize::from_ne_bytes(size.try_into().unwrap()),
                None => {
                    todo!()
                }
            };

            let slice_size = size_of::<usize>() + 1 + size + 4;

            if value.len() < slice_size {
                todo!()
            }

            let block_type = match BlockType::try_from(value[size_of::<usize>()]) {
                Ok(block_type) => block_type,
                Err(_) => todo!(),
            };
            let content = &value[size_of::<usize>() + 1..size_of::<usize>() + 1 + size];
            let block_crc =
                u32::from_ne_bytes(value[slice_size - 4..slice_size].try_into().unwrap());

            let crc = Crc::<u32>::new(&CRC_ALG);
            if crc.checksum(&value[..slice_size - 4]) != block_crc {
                todo!()
            }

            Ok(Block {
                block_type,
                content: Vec::from(content),
            })
        }
    }

    impl From<Block> for Vec<u8> {
        fn from(value: Block) -> Self {
            let mut out: Self = Self::with_capacity(value.len());

            out.extend_from_slice(&value.content.len().to_ne_bytes());
            out.push(value.block_type as u8);
            out.extend_from_slice(&value.content);
            out.extend_from_slice(&Crc::<u32>::new(&CRC_ALG).checksum(&out[..]).to_ne_bytes());

            out
        }
    }

    pub enum BlockType {
        Header,
        String,
    }

    impl TryFrom<u8> for BlockType {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                val if val == Self::Header as u8 => Ok(Self::Header),
                val if val == Self::String as u8 => Ok(Self::String),
                _ => Err(()),
            }
        }
    }
}
