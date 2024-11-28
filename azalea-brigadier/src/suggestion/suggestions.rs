#[cfg(feature = "azalea-buf")]
use std::io::{Cursor, Write};
use std::{collections::HashSet, hash::Hash};

#[cfg(feature = "azalea-buf")]
use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
#[cfg(feature = "azalea-buf")]
use azalea_chat::FormattedText;

use super::Suggestion;
use crate::context::StringRange;
#[cfg(feature = "azalea-buf")]
use crate::suggestion::SuggestionValue;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Suggestions {
    range: StringRange,
    suggestions: Vec<Suggestion>,
}

impl Suggestions {
    pub fn new(range: StringRange, suggestions: Vec<Suggestion>) -> Self {
        Self { range, suggestions }
    }

    pub fn merge(command: &str, input: &[Suggestions]) -> Self {
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

    pub fn create(command: &str, suggestions: &HashSet<Suggestion>) -> Self {
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
            texts.insert(suggestion.expand(command, range));
        }
        let mut sorted = texts.into_iter().collect::<Vec<_>>();

        sorted.sort_by(|a, b| a.value.cmp_ignore_case(&b.value));

        Suggestions {
            range,
            suggestions: sorted,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    pub fn list(&self) -> &[Suggestion] {
        &self.suggestions
    }

    pub fn range(&self) -> StringRange {
        self.range
    }
}

#[cfg(feature = "azalea-buf")]
impl AzaleaRead for Suggestions {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        #[derive(AzBuf)]
        struct StandaloneSuggestion {
            pub text: String,
            pub tooltip: Option<FormattedText>,
        }

        let start = u32::azalea_read_var(buf)? as usize;
        let length = u32::azalea_read_var(buf)? as usize;
        let range = StringRange::between(start, start + length);

        // the range of a Suggestion depends on the Suggestions containing it,
        // so we can't just `impl AzaleaRead for Suggestion`
        let mut suggestions = Vec::<StandaloneSuggestion>::azalea_read(buf)?
            .into_iter()
            .map(|s| Suggestion {
                value: SuggestionValue::Text(s.text),
                tooltip: s.tooltip.map(|t| t.to_string()),
                range,
            })
            .collect::<Vec<_>>();
        suggestions.sort_by(|a, b| a.value.cmp(&b.value));

        Ok(Suggestions { range, suggestions })
    }
}

#[cfg(feature = "azalea-buf")]
impl AzaleaWrite for Suggestions {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (self.range.start() as u32).azalea_write_var(buf)?;
        (self.range.length() as u32).azalea_write_var(buf)?;
        self.suggestions.azalea_write(buf)?;
        Ok(())
    }
}
