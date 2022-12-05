use super::Suggestion;
use crate::context::StringRange;
use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use std::collections::HashSet;
use std::io::{Cursor, Write};

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
        let range = StringRange::new(start, end);
        let mut texts = HashSet::new();
        for suggestion in suggestions {
            texts.insert(suggestion.expand(command, &range));
        }
        let mut sorted: Vec<Suggestion> = texts.into_iter().collect();
        sorted.sort_by(|a, b| a.text.cmp(&b.text));
        Suggestions {
            range,
            suggestions: sorted,
        }
    }
}

impl McBufReadable for Suggestions {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        buf.set_position(buf.position() + 1);
        let start = u32::var_read_from(buf)? as usize;
        let length = u32::var_read_from(buf)? as usize;
        let range = StringRange::between(start, start + length);

        let mut suggestions: Vec<Suggestion> = Vec::new();
        for _ in 0..length {
            suggestions.push(Suggestion::read_from(buf)?);
        }
        suggestions.sort_by(|a, b| a.text.cmp(&b.text));

        Ok(Suggestions { range, suggestions })
    }
}

impl McBufWritable for Suggestions {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_all(&[0])?;
        let start = self.range.start() as u32;
        start.var_write_into(buf)?;
        self.suggestions.write_into(buf)?;
        Ok(())
    }
}
