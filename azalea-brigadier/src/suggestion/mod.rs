mod suggestions;
mod suggestions_builder;

#[cfg(feature = "azalea-buf")]
use std::io::Write;
use std::{
    fmt::{self, Display},
    hash::Hash,
};

#[cfg(feature = "azalea-buf")]
use azalea_buf::McBufWritable;
#[cfg(feature = "azalea-buf")]
use azalea_chat::FormattedText;
pub use suggestions::Suggestions;
pub use suggestions_builder::SuggestionsBuilder;

use crate::context::StringRange;

/// A suggestion given to the user for what they might want to type next.
///
/// The `M` generic is the type of the tooltip, so for example a `String` or
/// just `()` if you don't care about it.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Suggestion {
    pub range: StringRange,
    value: SuggestionValue,
    pub tooltip: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum SuggestionValue {
    Integer(i32),
    Text(String),
}

impl Suggestion {
    pub fn new(range: StringRange, text: &str) -> Suggestion {
        Suggestion {
            range,
            value: SuggestionValue::Text(text.to_string()),
            tooltip: None,
        }
    }

    pub fn new_with_tooltip(range: StringRange, text: &str, tooltip: String) -> Self {
        Self {
            range,
            value: SuggestionValue::Text(text.to_string()),
            tooltip: Some(tooltip),
        }
    }

    pub fn apply(&self, input: &str) -> String {
        let text = self.value.to_string();
        if self.range.start() == 0 && self.range.end() == input.len() {
            return text;
        }
        let mut result = String::with_capacity(text.len());
        if self.range.start() > 0 {
            result.push_str(&input[0..self.range.start()]);
        }
        result.push_str(&text);
        if self.range.end() < input.len() {
            result.push_str(&input[self.range.end()..]);
        }

        result
    }

    pub fn expand(&self, command: &str, range: StringRange) -> Suggestion {
        if range == self.range {
            return self.clone();
        }
        let mut result = String::new();
        if range.start() < self.range.start() {
            result.push_str(&command[range.start()..self.range.start()]);
        }
        result.push_str(&self.value.to_string());
        if range.end() > self.range.end() {
            result.push_str(&command[self.range.end()..range.end()]);
        }
        Suggestion {
            range,
            value: SuggestionValue::Text(result),
            tooltip: self.tooltip.clone(),
        }
    }

    pub fn text(&self) -> String {
        self.value.to_string()
    }
}

impl SuggestionValue {
    pub fn cmp_ignore_case(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SuggestionValue::Text(a), SuggestionValue::Text(b)) => {
                a.to_lowercase().cmp(&b.to_lowercase())
            }
            (SuggestionValue::Integer(a), SuggestionValue::Integer(b)) => a.cmp(b),
            _ => {
                let a = self.to_string();
                let b = other.to_string();
                a.cmp(&b)
            }
        }
    }
}

impl Display for SuggestionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuggestionValue::Text(text) => write!(f, "{text}"),
            SuggestionValue::Integer(value) => write!(f, "{value}"),
        }
    }
}

impl Ord for SuggestionValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SuggestionValue::Text(a), SuggestionValue::Text(b)) => a.cmp(b),
            (SuggestionValue::Integer(a), SuggestionValue::Integer(b)) => a.cmp(b),
            _ => {
                let a = self.to_string();
                let b = other.to_string();
                a.cmp(&b)
            }
        }
    }
}
impl PartialOrd for SuggestionValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "azalea-buf")]
impl McBufWritable for Suggestion {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.value.to_string().write_into(buf)?;
        self.tooltip
            .clone()
            .map(FormattedText::from)
            .write_into(buf)?;
        Ok(())
    }
}
