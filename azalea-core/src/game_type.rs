use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::io::{Cursor, Write};

#[derive(Hash, Copy, Clone, Debug)]
pub enum GameType {
    SURVIVAL,
    CREATIVE,
    ADVENTURE,
    SPECTATOR,
}

impl GameType {
    pub fn to_id(&self) -> u8 {
        match self {
            GameType::SURVIVAL => 0,
            GameType::CREATIVE => 1,
            GameType::ADVENTURE => 2,
            GameType::SPECTATOR => 3,
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
            0 => GameType::SURVIVAL,
            1 => GameType::CREATIVE,
            2 => GameType::ADVENTURE,
            3 => GameType::SPECTATOR,
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
            GameType::SURVIVAL => "Survival",
            GameType::CREATIVE => "Creative",
            GameType::ADVENTURE => "Adventure",
            GameType::SPECTATOR => "Spectator",
        }
    }

    pub fn long_name(&self) -> &'static str {
        // TODO: These should be translated TranslatableComponent("gameMode." + string2);
        match self {
            GameType::SURVIVAL => "Survival Mode",
            GameType::CREATIVE => "Creative Mode",
            GameType::ADVENTURE => "Adventure Mode",
            GameType::SPECTATOR => "Spectator Mode",
        }
    }

    pub fn from_name(name: &str) -> GameType {
        match name {
            "survival" => GameType::SURVIVAL,
            "creative" => GameType::CREATIVE,
            "adventure" => GameType::ADVENTURE,
            "spectator" => GameType::SPECTATOR,
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
