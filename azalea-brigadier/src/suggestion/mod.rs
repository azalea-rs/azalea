mod suggestions;

use crate::{context::StringRange, message::Message};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Suggestion {
    pub range: StringRange,
    pub text: String,
    pub tooltip: Option<Message>,
}

impl Suggestion {
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
            result.push_str(&input[self.range.end()..])
        }

        result
    }

    pub fn expand(&self, command: &str, range: &StringRange) -> Suggestion {
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
