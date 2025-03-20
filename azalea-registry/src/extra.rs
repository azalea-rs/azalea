//! These registries are sent by the server during the configuration state so
//! you should be relying on those if possible, but these are provided for your
//! convenience anyways.

use azalea_registry_macros::registry;

use crate::Registry;

registry! {
#[derive(Default)]
enum FoxVariant {
    #[default]
    Red => "minecraft:red",
    Snow => "minecraft:snow",
}
}

registry! {
enum ParrotVariant {
    RedBlue => "minecraft:red_blue",
    Blue => "minecraft:blue",
    Green => "minecraft:green",
    YellowBlue => "minecraft:yellow_blue",
    Gray => "minecraft:gray",
}
}

registry! {
#[derive(Default)]
enum MooshroomVariant {
    #[default]
    Red => "minecraft:red",
    Brown => "minecraft:brown",
}
}

registry! {
#[derive(Default)]
enum RabbitVariant {
    #[default]
    Brown => "minecraft:brown",
    White => "minecraft:white",
    Black => "minecraft:black",
    WhiteSplotched => "minecraft:white_splotched",
    Gold => "minecraft:gold",
    Salt => "minecraft:salt",
    Evil => "minecraft:evil",
}
}

registry! {
#[derive(Default)]
enum HorseVariant {
    #[default]
    White => "minecraft:white",
    Creamy => "minecraft:creamy",
    Chestnut => "minecraft:chestnut",
    Brown => "minecraft:brown",
    Black => "minecraft:black",
    Gray => "minecraft:gray",
    DarkBrown => "minecraft:dark_brown",
}
}

registry! {
#[derive(Default)]
enum LlamaVariant {
    #[default]
    Creamy => "minecraft:creamy",
    White => "minecraft:white",
    Brown => "minecraft:brown",
    Gray => "minecraft:gray",
}
}

registry! {
#[derive(Default)]
enum AxolotlVariant {
    #[default]
    Lucy => "minecraft:lucy",
    Wild => "minecraft:wild",
    Gold => "minecraft:gold",
    Cyan => "minecraft:cyan",
    Blue => "minecraft:blue",
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

registry! {
enum Instrument {
    PonderGoatHorn => "minecraft:ponder_goat_horn",
    SingGoatHorn => "minecraft:sing_goat_horn",
    SeekGoatHorn => "minecraft:seek_goat_horn",
    FeelGoatHorn => "minecraft:feel_goat_horn",
    AdmireGoatHorn => "minecraft:admire_goat_horn",
    CallGoatHorn => "minecraft:call_goat_horn",
    YearnGoatHorn => "minecraft:yearn_goat_horn",
    DreamGoatHorn => "minecraft:dream_goat_horn",
}
}
