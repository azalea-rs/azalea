use std::cmp;

use crate::{context::string_range::StringRange, message::Message};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Suggestion {
    range: StringRange,
    text: String,
    tooltip: Option<Message>,
}

impl Suggestion {
    pub fn new(range: StringRange, text: String) -> Suggestion {
        Suggestion {
            range,
            text,
            tooltip: None,
        }
    }

    pub fn new_with_tooltip(range: StringRange, text: String, tooltip: Message) -> Suggestion {
        Suggestion {
            range,
            text,
            tooltip: Some(tooltip),
        }
    }

    pub fn range(&self) -> &StringRange {
        &self.range
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn tooltip(&self) -> Option<&Message> {
        self.tooltip.as_ref()
    }

    pub fn apply(&self, input: &str) -> String {
        if self.range.start() == 0 && self.range.end() == input.len() {
            return self.text.clone();
        }
        let mut result = String::new();
        if self.range.start() > 0 {
            result.push_str(&input[0..self.range.start()]);
        }
        result.push_str(&self.text);
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
        result.push_str(&self.text);
        if range.end() > self.range.end() {
            result.push_str(&command[self.range.end()..range.end()]);
        }
        Suggestion {
            range,
            text: result,
            tooltip: self.tooltip.clone(),
        }
    }

    pub fn compare_ignore_case(&self, b: &Suggestion) -> cmp::Ordering {
        self.text.to_lowercase().cmp(&b.text.to_lowercase())
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Suggestion) -> Option<cmp::Ordering> {
        Some(self.text.cmp(&other.text))
    }
}

impl Ord for Suggestion {
    fn cmp(&self, other: &Suggestion) -> cmp::Ordering {
        self.text.cmp(&other.text)
    }
}
