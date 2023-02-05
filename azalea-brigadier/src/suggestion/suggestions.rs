use super::Suggestion;
use crate::context::StringRange;
#[cfg(feature = "azalea-buf")]
use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
#[cfg(feature = "azalea-buf")]
use azalea_chat::FormattedText;
#[cfg(feature = "azalea-buf")]
use std::io::{Cursor, Write};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Suggestions<M = String> {
    pub range: StringRange,
    pub suggestions: Vec<Suggestion<M>>,
}

impl<M: Clone + Eq + Hash> Suggestions<M> {
    pub fn merge(command: &str, input: &[Suggestions<M>]) -> Self {
        if input.is_empty() {
            return Suggestions::default();
        } else if input.len() == 1 {
            return input[0].clone();
        };

        let mut texts = HashSet::new();
        for suggestions in input {
            texts.extend(suggestions.suggestions.clone());
        }

        Suggestions::create(command, &texts)
    }

    pub fn create(command: &str, suggestions: &HashSet<Suggestion<M>>) -> Self {
        if suggestions.is_empty() {
            return Suggestions::default();
        };
        let mut start = usize::MAX;
        let mut end = usize::MIN;
        for suggestion in suggestions {
            start = suggestion.range.start().min(start);
            end = suggestion.range.end().max(end);
        }
        let range = StringRange::new(start, end);
        let mut texts = HashSet::new();
        for suggestion in suggestions {
            texts.insert(suggestion.expand(command, &range));
        }
        let mut sorted = texts.into_iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.text.cmp(&b.text));
        Suggestions {
            range,
            suggestions: sorted,
        }
    }
}

// this can't be derived because that'd require the generic to have `Default`
// too even if it's not actually necessary
impl<M> Default for Suggestions<M> {
    fn default() -> Self {
        Self {
            range: StringRange::default(),
            suggestions: Vec::new(),
        }
    }
}

#[cfg(feature = "azalea-buf")]
impl McBufReadable for Suggestions<FormattedText> {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        #[derive(McBuf)]
        struct StandaloneSuggestion {
            pub text: String,
            pub tooltip: Option<FormattedText>,
        }

        let start = u32::var_read_from(buf)? as usize;
        let length = u32::var_read_from(buf)? as usize;
        let range = StringRange::between(start, start + length);

        // the range of a Suggestion depends on the Suggestions containing it,
        // so we can't just `impl McBufReadable for Suggestion`
        let mut suggestions = Vec::<StandaloneSuggestion>::read_from(buf)?
            .into_iter()
            .map(|s| Suggestion {
                text: s.text,
                tooltip: s.tooltip,
                range: range.clone(),
            })
            .collect::<Vec<_>>();
        suggestions.sort_by(|a, b| a.text.cmp(&b.text));

        Ok(Suggestions { range, suggestions })
    }
}

#[cfg(feature = "azalea-buf")]
impl McBufWritable for Suggestions<FormattedText> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.range.start() as u32).var_write_into(buf)?;
        (self.range.length() as u32).var_write_into(buf)?;
        self.suggestions.write_into(buf)?;
        Ok(())
    }
}
