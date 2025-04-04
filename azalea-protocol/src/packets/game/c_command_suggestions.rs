use azalea_brigadier::suggestion::Suggestions;
use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundCommandSuggestions {
    #[var]
    pub id: u32,
    pub suggestions: Suggestions,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_brigadier::{context::StringRange, suggestion::Suggestion};
    use azalea_buf::{AzaleaRead, AzaleaWrite};

    use super::*;

    #[test]
    fn test_suggestions() {
        let suggestions = Suggestions::new(
            StringRange::new(0, 5),
            vec![Suggestion::new_with_tooltip(
                StringRange::new(1, 4),
                "foo",
                "bar".to_string(),
            )],
        );
        let mut buf = Vec::new();
        suggestions.azalea_write(&mut buf).unwrap();
        let mut cursor = Cursor::new(&buf[..]);
        let suggestions = Suggestions::azalea_read(&mut cursor).unwrap();
        assert_eq!(suggestions, suggestions);
    }
}
