use std::{cmp, collections::HashSet};

use crate::{context::string_range::StringRange, message::Message};

use super::suggestion::Suggestion;

#[derive(PartialEq, Eq, Hash, Default)]
pub struct Suggestions {
    range: StringRange,
    suggestions: Vec<Suggestions>,
}

impl Suggestions {
    fn range(&self) -> &StringRange {
        &self.range
    }

    fn list(&self) -> &Vec<Suggestions> {
        &self.suggestions
    }

    fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    fn merge(command: &str, input: &Vec<Suggestions>) {
        if input.is_empty() {
            return Self::default();
        } else if input.len() == 1 {
            return input.iter().next();
        }
        let texts = HashSet::new();
        for suggestions in input {
            texts.extend(suggestions.list())
        }
        Self::new(command, texts)
    }

    // public static Suggestions create(final String command, final Collection<Suggestion> suggestions) {
    //     if (suggestions.isEmpty()) {
    //         return EMPTY;
    //     }
    //     int start = Integer.MAX_VALUE;
    //     int end = Integer.MIN_VALUE;
    //     for (final Suggestion suggestion : suggestions) {
    //         start = Math.min(suggestion.getRange().getStart(), start);
    //         end = Math.max(suggestion.getRange().getEnd(), end);
    //     }
    //     final StringRange range = new StringRange(start, end);
    //     final Set<Suggestion> texts = new HashSet<>();
    //     for (final Suggestion suggestion : suggestions) {
    //         texts.add(suggestion.expand(command, range));
    //     }
    //     final List<Suggestion> sorted = new ArrayList<>(texts);
    //     sorted.sort((a, b) -> a.compareToIgnoreCase(b));
    //     return new Suggestions(range, sorted);
    pub fn new(command: String, suggestions: Vec<Suggestion>) -> Self {
        if suggestions.is_empty() {
            return Self::default();
        }
        let mut start = usize::MAX;
        let mut end = usize::MIN;
        for suggestion in suggestions {
            let start = cmp::min(suggestion.range().start(), start);
            let end = cmp::max(suggestion.range().end(), end);
        }
        let range = StringRange::new(start, end);
        let texts = HashSet::new();
        for suggestion in suggestions {
            texts.insert(suggestion.expand(command, range));
        }
        let sorted = texts.sort_by(|a, b| a.compare_ignore_case(b));
        Suggestions {
            range,
            suggestions: sorted,
        }
    }
}

impl Default for Suggestions {
    fn default() -> Self {
        Self {
            range: StringRange::at(0),
            suggestions: vec![],
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::suggestion::suggestion::Suggestion;

//     use super::*;

//     #[test]
//     fn merge_empty() {
//         let merged = Suggestions::merge("foo b", vec![]);
//         assert_eq!(merged.is_empty(), true);
//     }

//     #[test]
//     fn merge_single() {
//         let suggestions = Suggestions::new(StringRange::at(5), "ar".to_string());
//         let merged = Suggestions::merge("foo b", vec![suggestions]);
//         assert_eq!(merged, suggestions);
//     }

//     #[test]
//     fn merge_multiple() {
//         let a = Suggestions::new(
//             StringRange::at(5),
//             vec![
//                 Suggestion::new(StringRange::at(5), "ar".to_string()),
//                 Suggestion::new(StringRange::at(5), "az".to_string()),
//                 Suggestion::new(StringRange::at(5), "Az".to_string()),
//             ],
//         );
//         let b = Suggestions::new(
//             StringRange::between(4, 5),
//             vec![
//                 Suggestion::new(StringRange::between(4, 5), "foo".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "qux".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "apple".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "Bar".to_string()),
//             ],
//         );
//         let merged = Suggestions::merge("foo b", vec![a, b]);
//         assert_eq!(
//             merged.get_list(),
//             vec![
//                 Suggestion::new(StringRange::between(4, 5), "apple".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "bar".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "Bar".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "baz".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "bAz".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "foo".to_string()),
//                 Suggestion::new(StringRange::between(4, 5), "qux".to_string()),
//             ]
//         );
//     }
// }
