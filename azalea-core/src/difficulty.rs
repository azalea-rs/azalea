use std::{
    fmt::{self, Debug},
    io::{self, Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaWrite, BufReadError};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

pub struct InvalidDifficultyError(pub String);

impl Debug for InvalidDifficultyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid difficulty: {}", self.0)
    }
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Peaceful => "peaceful",
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Hard => "hard",
        }
    }

    pub fn from_name(name: &str) -> Result<Difficulty, InvalidDifficultyError> {
        Ok(match name {
            "peaceful" => Difficulty::Peaceful,
            "easy" => Difficulty::Easy,
            "normal" => Difficulty::Normal,
            "hard" => Difficulty::Hard,
            _ => return Err(InvalidDifficultyError(name.to_owned())),
        })
    }

    pub fn by_id(id: u8) -> Difficulty {
        match id % 4 {
            0 => Difficulty::Peaceful,
            1 => Difficulty::Easy,
            2 => Difficulty::Normal,
            3 => Difficulty::Hard,
            _ => unreachable!(),
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

impl AzaleaRead for Difficulty {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(Difficulty::by_id(u8::azalea_read(buf)?))
    }
}

impl AzaleaWrite for Difficulty {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        u8::azalea_write(&self.id(), buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_from_name() {
        assert_eq!(
            Difficulty::Peaceful,
            Difficulty::from_name("peaceful").unwrap()
        );
        assert_eq!(Difficulty::Easy, Difficulty::from_name("easy").unwrap());
        assert_eq!(Difficulty::Normal, Difficulty::from_name("normal").unwrap());
        assert_eq!(Difficulty::Hard, Difficulty::from_name("hard").unwrap());
        assert!(Difficulty::from_name("invalid").is_err());
    }

    #[test]
    fn test_difficulty_id() {
        assert_eq!(0, Difficulty::Peaceful.id());
        assert_eq!(1, Difficulty::Easy.id());
        assert_eq!(2, Difficulty::Normal.id());
        assert_eq!(3, Difficulty::Hard.id());
    }

    #[test]
    fn test_difficulty_name() {
        assert_eq!("peaceful", Difficulty::Peaceful.name());
        assert_eq!("easy", Difficulty::Easy.name());
        assert_eq!("normal", Difficulty::Normal.name());
        assert_eq!("hard", Difficulty::Hard.name());
    }
}
