use super::Suggestion;
use crate::context::StringRange;
use std::collections::HashSet;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Suggestions {
    pub range: StringRange,
    pub suggestions: Vec<Suggestion>,
}

impl Suggestions {
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
        todo!()
    }
}
