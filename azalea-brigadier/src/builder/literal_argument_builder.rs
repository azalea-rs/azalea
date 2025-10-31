use super::argument_builder::{ArgumentBuilder, ArgumentBuilderType};

#[derive(Debug, Clone, Default)]
pub struct Literal {
    pub value: String,
}
impl Literal {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl<S> From<Literal> for ArgumentBuilderType<S> {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

/// Shortcut for creating a new literal builder node.
#[must_use]
pub fn literal<S>(value: &str) -> ArgumentBuilder<S> {
    ArgumentBuilder::new(ArgumentBuilderType::Literal(Literal::new(value)))
}
