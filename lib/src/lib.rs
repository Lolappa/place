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
