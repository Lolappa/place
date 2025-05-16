#[allow(unused)]

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

    pub const CRC_ALG: Algorithm<u32> = crc::CRC_24_OPENPGP;

    pub struct Packet {
        blocks: Vec<Block>,
    }

    impl Packet {
        pub fn blocks(&self) -> &[Block] {
            &self.blocks
        }

        pub fn blocks_mut(&mut self) -> &mut Vec<Block> {
            &mut self.blocks
        }
    }

    impl TryFrom<&[u8]> for Packet {
        type Error = (); // TODO: use an actual type here

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let mut blocks: Vec<Block> = vec![];

            let mut slice = &value[..];

            while slice.len() > 0 {
                let (block, size) = match Block::slice_from(slice) {
                    Ok(value) => value,
                    Err(_) => {
                        todo!()
                    }
                };
            }

            Ok(Packet { blocks })
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

        fn slice_from(value: &[u8]) -> Result<(Self, usize), ()> {
            // TODO: use an actual error type here
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

            Ok((
                Block {
                    block_type,
                    content: Vec::from(content),
                },
                slice_size,
            ))
        }
    }

    pub enum BlockType {
        Meta,
        String,
    }

    impl TryFrom<u8> for BlockType {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                val if val == Self::Meta as u8 => Ok(Self::Meta),
                val if val == Self::String as u8 => Ok(Self::String),
                _ => Err(()),
            }
        }
    }
}
