//! These registries are sent by the server during the configuration state so
//! you should be relying on those if possible, but these are provided for your
//! convenience anyways.

use azalea_registry_macros::registry;

use crate::Registry;

registry! {
enum WolfVariant {
    Pale => "minecraft:wolf",
    Spotted => "minecraft:wolf_spotted",
    Snowy => "minecraft:wolf_snowy",
    Black => "minecraft:wolf_black",
    Ashen => "minecraft:wolf_ashen",
    Rusty => "minecraft:wolf_rusty",
    Woods => "minecraft:wolf_woods",
    Chestnut => "minecraft:wolf_chestnut",
    Striped => "minecraft:wolf_striped",
}
}

#[allow(clippy::derivable_impls)]
impl Default for WolfVariant {
    fn default() -> Self {
        WolfVariant::Pale
    }
}

registry! {
enum DimensionType {
    Overworld => "minecraft:overworld",
    Nether => "minecraft:the_nether",
    End => "minecraft:the_end",
    OverworldCaves => "minecraft:overworld_caves",
}
}

registry! {
enum TrimMaterial {
    Quartz => "minecraft:quartz",
    Iron => "minecraft:iron",
    Netherite => "minecraft:netherite",
    Redstone => "minecraft:redstone",
    Copper => "minecraft:copper",
    Gold => "minecraft:gold",
    Emerald => "minecraft:emerald",
    Diamond => "minecraft:diamond",
    Lapis => "minecraft:lapis",
    Amethyst => "minecraft:amethyst",
}
}

registry! {
enum TrimPattern {
    Sentry => "sentry",
    Dune => "dune",
    Coast => "coast",
    Wild => "wild",
    Ward => "ward",
    Eye => "eye",
    Vex => "vex",
    Tide => "tide",
    Snout => "snout",
    Rib => "rib",
    Spire => "spire",
    Wayfinder => "wayfinder",
    Shaper => "shaper",
    Silence => "silence",
    Raiser => "raiser",
    Host => "host",
    Flow => "flow",
    Bolt => "bolt",
}
}

registry! {
enum JukeboxSong {
    Thirteen => "13",
    Cat => "cat",
    Blocks => "blocks",
    Chirp => "chirp",
    Far => "far",
    Mall => "mall",
    Mellohi => "mellohi",
    Stal => "stal",
    Strad => "strad",
    Ward => "ward",
    Eleven => "11",
    Wait => "wait",
    Pigstep => "pigstep",
    Otherside => "otherside",
    Five => "5",
    Relic => "relic",
    Precipice => "precipice",
    Creator => "creator",
    CreatorMusicBox => "creator_music_box",
}
}

registry! {
enum ChatType {
    Chat => "chat",
    SayCommand => "say_command",
    MsgCommandIncoming => "msg_command_incoming",
    MsgCommandOutgoing => "msg_command_outgoing",
    TeamMsgCommandIncoming => "team_msg_command_incoming",
    TeamMsgCommandOutgoing => "team_msg_command_outgoing",
    EmoteCommand => "emote_command",
}
}
impl ChatType {
    #[must_use]
    pub fn chat_translation_key(self) -> &'static str {
        match self {
            ChatType::Chat => "chat.type.text",
            ChatType::SayCommand => "chat.type.announcement",
            ChatType::MsgCommandIncoming => "commands.message.display.incoming",
            ChatType::MsgCommandOutgoing => "commands.message.display.outgoing",
            ChatType::TeamMsgCommandIncoming => "chat.type.team.text",
            ChatType::TeamMsgCommandOutgoing => "chat.type.team.sent",
            ChatType::EmoteCommand => "chat.type.emote",
        }
    }

    #[must_use]
    pub fn narrator_translation_key(self) -> &'static str {
        match self {
            ChatType::EmoteCommand => "chat.type.emote",
            _ => "chat.type.text.narrate",
        }
    }
}
