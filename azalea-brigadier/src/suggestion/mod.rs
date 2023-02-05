mod suggestions;

use crate::context::StringRange;
#[cfg(feature = "azalea-buf")]
use azalea_buf::McBufWritable;
#[cfg(feature = "azalea-buf")]
use azalea_chat::FormattedText;
#[cfg(feature = "azalea-buf")]
use std::io::Write;
pub use suggestions::*;

/// A suggestion given to the user for what they might want to type next.
///
/// The `M` generic is the type of the tooltip, so for example a `String` or
/// just `()` if you don't care about it.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Suggestion<M = String> {
    pub text: String,
    pub range: StringRange,
    pub tooltip: Option<M>,
}

impl<M: Clone> Suggestion<M> {
    pub fn apply(&self, input: &str) -> String {
        if self.range.start() == 0 && self.range.end() == input.len() {
            return input.to_string();
        }
        let mut result = String::with_capacity(self.text.len());
        if self.range.start() > 0 {
            result.push_str(&input[0..self.range.start()]);
        }
        result.push_str(&self.text);
        if self.range.end() < input.len() {
            result.push_str(&input[self.range.end()..]);
        }

        result
    }

    pub fn expand(&self, command: &str, range: &StringRange) -> Suggestion<M> {
        if range == &self.range {
            return self.clone();
        }
        let mut result = String::new();
        if range.start() < self.range.start() {
            result.push_str(&command[range.start()..self.range.start()]);
        }
        result.push_str(&self.text);
        if range.end() > self.range.end() {
            result.push_str(&command[self.range.end()..range.end()]);
        }
        Suggestion {
            range: range.clone(),
            text: result,
            tooltip: self.tooltip.clone(),
        }
    }
}

#[cfg(feature = "azalea-buf")]
impl McBufWritable for Suggestion<FormattedText> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.text.write_into(buf)?;
        self.tooltip.write_into(buf)?;
        Ok(())
    }
}
