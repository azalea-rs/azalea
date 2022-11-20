use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::io::{Cursor, Write};

#[derive(Hash, Copy, Clone, Debug)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameType {
    pub fn to_id(&self) -> u8 {
        match self {
            GameType::Survival => 0,
            GameType::Creative => 1,
            GameType::Adventure => 2,
            GameType::Spectator => 3,
        }
    }

    /// Get the id of the game type, but return -1 if the game type is invalid.
    pub fn to_optional_id<T: Into<Option<GameType>>>(game_type: T) -> i8 {
        match game_type.into() {
            Some(game_type) => game_type.to_id() as i8,
            None => -1,
        }
    }

    pub fn from_id(id: u8) -> Option<GameType> {
        Some(match id {
            0 => GameType::Survival,
            1 => GameType::Creative,
            2 => GameType::Adventure,
            3 => GameType::Spectator,
            _ => return None,
        })
    }

    pub fn from_optional_id(id: i8) -> Option<OptionalGameType> {
        Some(
            match id {
                -1 => None,
                id => Some(GameType::from_id(id as u8)?),
            }
            .into(),
        )
    }

    pub fn short_name(&self) -> &'static str {
        // TODO: these should be translated TranslatableComponent("selectWorld.gameMode." + string2)
        match self {
            GameType::Survival => "Survival",
            GameType::Creative => "Creative",
            GameType::Adventure => "Adventure",
            GameType::Spectator => "Spectator",
        }
    }

    pub fn long_name(&self) -> &'static str {
        // TODO: These should be translated TranslatableComponent("gameMode." + string2);
        match self {
            GameType::Survival => "Survival Mode",
            GameType::Creative => "Creative Mode",
            GameType::Adventure => "Adventure Mode",
            GameType::Spectator => "Spectator Mode",
        }
    }

    pub fn from_name(name: &str) -> GameType {
        match name {
            "survival" => GameType::Survival,
            "creative" => GameType::Creative,
            "adventure" => GameType::Adventure,
            "spectator" => GameType::Spectator,
            _ => panic!("Unknown game type name: {name}"),
        }
    }
}

impl McBufReadable for GameType {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u8::read_from(buf)?;
        GameType::from_id(id).ok_or(BufReadError::UnexpectedEnumVariant { id: id as i32 })
    }
}

impl McBufWritable for GameType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u8::write_into(&self.to_id(), buf)
    }
}

/// Rust doesn't let us `impl McBufReadable for Option<GameType>` so we have to make a new type :(
#[derive(Hash, Copy, Clone, Debug)]
pub struct OptionalGameType(Option<GameType>);

impl From<Option<GameType>> for OptionalGameType {
    fn from(game_type: Option<GameType>) -> Self {
        OptionalGameType(game_type)
    }
}

impl From<OptionalGameType> for Option<GameType> {
    fn from(optional_game_type: OptionalGameType) -> Self {
        optional_game_type.0
    }
}

impl McBufReadable for OptionalGameType {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i8::read_from(buf)?;
        GameType::from_optional_id(id).ok_or(BufReadError::UnexpectedEnumVariant { id: id as i32 })
    }
}

impl McBufWritable for OptionalGameType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        GameType::to_optional_id(*self).write_into(buf)
    }
}
