use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
use azalea_core::bitset::FixedBitSet;
use bevy_ecs::component::Component;

/// A component that contains some of the "settings" for this client that are
/// sent to the server, such as render distance. This is only present on local
/// players.
#[derive(Clone, Debug, AzBuf, PartialEq, Eq, Component)]
pub struct ClientInformation {
    /// The locale of the client.
    pub language: String,
    /// The view distance of the client in chunks, same as the render distance
    /// in-game.
    pub view_distance: u8,
    /// The types of chat messages the client wants to receive. Note that many
    /// servers ignore this.
    pub chat_visibility: ChatVisibility,
    /// Whether the messages sent from the server should have colors. Note that
    /// many servers ignore this and always send colored messages.
    pub chat_colors: bool,
    pub model_customization: ModelCustomization,
    pub main_hand: HumanoidArm,
    pub text_filtering_enabled: bool,
    /// Whether the client should show up as "Anonymous Player" in the server
    /// list.
    pub allows_listing: bool,
    pub particle_status: ParticleStatus,
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self {
            language: "en_us".to_string(),
            view_distance: 8,
            chat_visibility: ChatVisibility::default(),
            chat_colors: true,
            model_customization: ModelCustomization::default(),
            main_hand: HumanoidArm::Right,
            text_filtering_enabled: false,
            allows_listing: false,
            particle_status: ParticleStatus::default(),
        }
    }
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChatVisibility {
    /// All chat messages should be sent to the client.
    #[default]
    Full = 0,
    /// Chat messages from other players should be not sent to the client, only
    /// messages from the server like "Player joined the game" should be sent.
    System = 1,
    /// No chat messages should be sent to the client.
    Hidden = 2,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HumanoidArm {
    Left = 0,
    #[default]
    Right = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelCustomization {
    pub cape: bool,
    pub jacket: bool,
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants: bool,
    pub right_pants: bool,
    pub hat: bool,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ParticleStatus {
    #[default]
    All,
    Decreased,
    Minimal,
}

impl Default for ModelCustomization {
    fn default() -> Self {
        Self {
            cape: true,
            jacket: true,
            left_sleeve: true,
            right_sleeve: true,
            left_pants: true,
            right_pants: true,
            hat: true,
        }
    }
}

impl AzaleaRead for ModelCustomization {
    fn azalea_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let set = FixedBitSet::<{ 7_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(Self {
            cape: set.index(0),
            jacket: set.index(1),
            left_sleeve: set.index(2),
            right_sleeve: set.index(3),
            left_pants: set.index(4),
            right_pants: set.index(5),
            hat: set.index(6),
        })
    }
}

impl AzaleaWrite for ModelCustomization {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 7_usize.div_ceil(8) }>::new();
        if self.cape {
            set.set(0);
        }
        if self.jacket {
            set.set(1);
        }
        if self.left_sleeve {
            set.set(2);
        }
        if self.right_sleeve {
            set.set(3);
        }
        if self.left_pants {
            set.set(4);
        }
        if self.right_pants {
            set.set(5);
        }
        if self.hat {
            set.set(6);
        }
        set.azalea_write(buf)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::{AzaleaRead, AzaleaWrite};

    use super::*;

    #[test]
    fn test_client_information() {
        {
            let data = ClientInformation::default();
            let mut buf = Vec::new();
            data.azalea_write(&mut buf).unwrap();
            let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);

            let read_data = ClientInformation::azalea_read(&mut data_cursor).unwrap();
            assert_eq!(read_data, data);
        }

        let data = ClientInformation {
            language: "en_gb".to_string(),
            view_distance: 24,
            chat_visibility: ChatVisibility::Hidden,
            chat_colors: false,
            model_customization: ModelCustomization {
                cape: false,
                jacket: false,
                left_sleeve: true,
                right_sleeve: false,
                left_pants: true,
                right_pants: false,
                hat: true,
            },
            main_hand: HumanoidArm::Left,
            text_filtering_enabled: true,
            allows_listing: true,
            particle_status: ParticleStatus::Decreased,
        };
        let mut buf = Vec::new();
        data.azalea_write(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);

        let read_data = ClientInformation::azalea_read(&mut data_cursor).unwrap();
        assert_eq!(read_data, data);
    }
}
