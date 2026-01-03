use super::argument_builder::{ArgumentBuilder, ArgumentBuilderType};

#[derive(Clone, Debug, Default)]
pub struct Literal {
    pub value: String,
}
impl Literal {
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
pub fn literal<S>(value: &str) -> ArgumentBuilder<S> {
    ArgumentBuilder::new(ArgumentBuilderType::Literal(Literal::new(value)))
}
