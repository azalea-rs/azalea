use azalea_chat;

#[derive(Hash, Clone, Debug)]
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
    pub fn to_optional_id(game_type: &Option<GameType>) -> i8 {
        match game_type {
            Some(game_type) => game_type.to_id() as i8,
            None => -1,
        }
    }

    pub fn from_id(id: u8) -> GameType {
        match id {
            0 => GameType::SURVIVAL,
            1 => GameType::CREATIVE,
            2 => GameType::ADVENTURE,
            3 => GameType::SPECTATOR,
            _ => panic!("Unknown game type id: {}", id),
        }
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
            _ => panic!("Unknown game type name: {}", name),
        }
    }
}
