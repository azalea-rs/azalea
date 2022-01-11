use std::cmp;

use crate::{context::string_range::StringRange, message::Message};

pub struct Suggestions {}

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
