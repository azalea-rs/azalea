use azalea_brigadier::suggestion::Suggestions;
use azalea_buf::McBuf;
use azalea_chat::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    pub suggestions: Suggestions<Component>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_brigadier::{context::StringRange, suggestion::Suggestion};
    use azalea_buf::{McBufReadable, McBufWritable};
    use std::io::Cursor;

    #[test]
    fn test_suggestions() {
        let suggestions = Suggestions {
            range: StringRange::new(0, 5),
            suggestions: vec![Suggestion {
                text: "foo".to_string(),
                range: StringRange::new(1, 4),
                tooltip: Some(Component::from("bar".to_string())),
            }],
        };
        let mut buf = Vec::new();
        suggestions.write_into(&mut buf).unwrap();
        let mut cursor = Cursor::new(&buf[..]);
        let suggestions = Suggestions::read_from(&mut cursor).unwrap();
        assert_eq!(suggestions, suggestions);
    }
}
