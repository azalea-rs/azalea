use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufWritable};
use tracing::debug;

/// A Minecraft gamemode, like survival or creative.
#[derive(Hash, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum GameMode {
    #[default]
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub fn to_id(&self) -> u8 {
        match self {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        }
    }

    /// Get the id of the game type, but return -1 if the game type is invalid.
    pub fn to_optional_id<T: Into<Option<GameMode>>>(game_type: T) -> i8 {
        match game_type.into() {
            Some(game_type) => game_type.to_id() as i8,
            None => -1,
        }
    }

    pub fn from_id(id: u8) -> Option<GameMode> {
        Some(match id {
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => return None,
        })
    }

    pub fn from_optional_id(id: i8) -> Option<OptionalGameType> {
        Some(
            match id {
                -1 => None,
                id => Some(GameMode::from_id(id as u8)?),
            }
            .into(),
        )
    }

    pub fn short_name(&self) -> &'static str {
        // TODO: these should be translated
        // TranslatableComponent("selectWorld.gameMode." + string2)
        match self {
            GameMode::Survival => "Survival",
            GameMode::Creative => "Creative",
            GameMode::Adventure => "Adventure",
            GameMode::Spectator => "Spectator",
        }
    }

    pub fn long_name(&self) -> &'static str {
        // TODO: These should be translated TranslatableComponent("gameMode." +
        // string2);
        match self {
            GameMode::Survival => "Survival Mode",
            GameMode::Creative => "Creative Mode",
            GameMode::Adventure => "Adventure Mode",
            GameMode::Spectator => "Spectator Mode",
        }
    }

    pub fn from_name(name: &str) -> GameMode {
        match name {
            "survival" => GameMode::Survival,
            "creative" => GameMode::Creative,
            "adventure" => GameMode::Adventure,
            "spectator" => GameMode::Spectator,
            _ => panic!("Unknown game type name: {name}"),
        }
    }
}

impl GameMode {
    /// Whether the player can't interact with blocks while in this game mode.
    ///
    /// (Returns true if you're in adventure or spectator.)
    pub fn is_block_placing_restricted(&self) -> bool {
        matches!(self, GameMode::Adventure | GameMode::Spectator)
    }
}

impl McBufReadable for GameMode {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        let id = id.try_into().unwrap_or_else(|_| {
            debug!("Unknown game mode id {id}, defaulting to survival");
            0
        });
        Ok(GameMode::from_id(id).unwrap_or_else(|| {
            debug!("Unknown game mode id {id}, defaulting to survival");
            GameMode::Survival
        }))
    }
}

impl McBufWritable for GameMode {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u8::write_into(&self.to_id(), buf)
    }
}

/// Rust doesn't let us `impl McBufReadable for Option<GameType>` so we have to
/// make a new type :(
#[derive(Hash, Copy, Clone, Debug)]
pub struct OptionalGameType(pub Option<GameMode>);

impl From<Option<GameMode>> for OptionalGameType {
    fn from(game_type: Option<GameMode>) -> Self {
        OptionalGameType(game_type)
    }
}

impl From<OptionalGameType> for Option<GameMode> {
    fn from(optional_game_type: OptionalGameType) -> Self {
        optional_game_type.0
    }
}

impl McBufReadable for OptionalGameType {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i8::read_from(buf)?;
        GameMode::from_optional_id(id).ok_or(BufReadError::UnexpectedEnumVariant { id: id as i32 })
    }
}

impl McBufWritable for OptionalGameType {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        GameMode::to_optional_id(*self).write_into(buf)
    }
}
