use crate::context::string_range::StringRange;

use super::{suggestion::Suggestion, suggestions::Suggestions};

pub struct SuggestionsBuilder {
    input: String,
    input_lowercase: String,
    start: usize,
    remaining: String,
    remaining_lowercase: String,
    result: Vec<Suggestion>,
}

impl SuggestionsBuilder {
    pub fn new_with_lowercase(
        input: String,
        input_lowercase: String,
        start: usize,
    ) -> SuggestionsBuilder {
        SuggestionsBuilder {
            input,
            input_lowercase,
            start,
            remaining: input.get(start..).unwrap().to_string(),
            remaining_lowercase: input_lowercase.get(start..).unwrap().to_string(),
            result: Vec::new(),
        }
    }

    pub fn new(input: String, start: usize) -> SuggestionsBuilder {
        SuggestionsBuilder::new_with_lowercase(input, input.to_lowercase(), start)
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn remaining(&self) -> &str {
        &self.remaining
    }

    pub fn remaining_lowercase(&self) -> &str {
        &self.remaining_lowercase
    }

    pub fn build(&self) -> Suggestions {
        Suggestions::create(self.input(), self.result)
    }

    pub fn suggest(&mut self, text: &str) -> &mut SuggestionsBuilder {
        if text == self.remaining {
            return self;
        }
        self.result.push(Suggestion::new(
            StringRange::between(self.start, self.input.len()),
            text,
        ));
        self
    }

    pub fn suggest_with_tooltip(&mut self, text: &str, tooltip: &str) -> &mut SuggestionsBuilder {
        if text == self.remaining {
            return self;
        }
        self.result.push(Suggestion::new_with_tooltip(
            StringRange::between(self.start, self.input.len()),
            text,
            tooltip,
        ));
        self
    }

    pub fn suggest_with_value(&mut self, value: i32) -> &mut SuggestionsBuilder {
        self.result.push(IntegerSuggestion::new(
            StringRange::between(self.start, self.input.len()),
            value,
        ));
        self
    }

    pub fn suggest_with_value_and_tooltip(
        &mut self,
        value: i32,
        tooltip: &str,
    ) -> &mut SuggestionsBuilder {
        self.result.push(IntegerSuggestion::new_with_tooltip(
            StringRange::between(self.start, self.input.len()),
            value,
            tooltip,
        ));
        self
    }

    pub fn add(&mut self, other: &SuggestionsBuilder) -> &mut SuggestionsBuilder {
        self.result.extend(other.result.iter().cloned());
        self
    }

    pub fn create_offset(&self, start: usize) -> SuggestionsBuilder {
        SuggestionsBuilder::new_with_lowercase(
            self.input.clone(),
            self.input_lowercase.clone(),
            start,
        )
    }

    pub fn restart(&self) -> SuggestionsBuilder {
        self.create_offset(self.start)
    }
}
