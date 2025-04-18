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
    }

    impl TryFrom<&[u8]> for Block {
        type Error = (); // TODO: use an actual type here

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            let size = match value.get(..size_of::<usize>()) {
                Some(size) => usize::from_ne_bytes(size.try_into().unwrap()),
                None => {
                    todo!()
                }
            };

            if value.len() < size_of::<usize>() + 1 + size + 4 {
                todo!()
            }

            let block_type = match BlockType::try_from(value[size_of::<usize>()]) {
                Ok(block_type) => block_type,
                Err(_) => todo!(),
            };
            let content = &value[size_of::<usize>() + 1..size_of::<usize>() + 1 + size];
            let block_crc = u32::from_ne_bytes(
                value[size_of::<usize>() + 1 + size..size_of::<usize>() + 1 + size + 4]
                    .try_into()
                    .unwrap(),
            );

            let crc = Crc::<u32>::new(&CRC_ALG);
            if crc.checksum(content) != block_crc {
                todo!()
            }

            Ok(Block {
                block_type,
                content: Vec::from(content),
            })
        }
    }

    pub enum BlockType {
        Meta = 1,
        String = 2,
    }

    impl TryFrom<u8> for BlockType {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                1 => Ok(Self::Meta),
                2 => Ok(Self::String),
                _ => Err(()),
            }
        }
    }
}
