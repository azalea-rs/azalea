use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    ptr,
    sync::Arc,
};

use parking_lot::RwLock;

use crate::{
    builder::{
        argument_builder::ArgumentBuilderType, literal_argument_builder::Literal,
        required_argument_builder::Argument,
    },
    context::{CommandContext, CommandContextBuilder, ParsedArgument, StringRange},
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
};

pub type Command<S> = Option<Arc<dyn Fn(&CommandContext<S>) -> i32 + Send + Sync>>;

/// An ArgumentBuilder that has been built.
#[non_exhaustive]
pub struct CommandNode<S> {
    pub value: ArgumentBuilderType,

    // this is a BTreeMap because children need to be ordered when getting command suggestions
    pub children: BTreeMap<String, Arc<RwLock<CommandNode<S>>>>,
    pub literals: HashMap<String, Arc<RwLock<CommandNode<S>>>>,
    pub arguments: HashMap<String, Arc<RwLock<CommandNode<S>>>>,

    pub command: Command<S>,
    pub requirement: Arc<dyn Fn(&S) -> bool + Send + Sync>,
    pub redirect: Option<Arc<RwLock<CommandNode<S>>>>,
    pub forks: bool,
    pub modifier: Option<Arc<RedirectModifier<S>>>,
}

impl<S> Clone for CommandNode<S> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone(),
            literals: self.literals.clone(),
            arguments: self.arguments.clone(),
            command: self.command.clone(),
            requirement: self.requirement.clone(),
            redirect: self.redirect.clone(),
            forks: self.forks,
            modifier: self.modifier.clone(),
        }
    }
}

impl<S> CommandNode<S> {
    /// Gets the literal, or panics. You should use match if you're not certain
    /// about the type.
    pub fn literal(&self) -> &Literal {
        match self.value {
            ArgumentBuilderType::Literal(ref literal) => literal,
            _ => panic!("CommandNode::literal() called on non-literal node"),
        }
    }
    /// Gets the argument, or panics. You should use match if you're not certain
    /// about the type.
    pub fn argument(&self) -> &Argument {
        match self.value {
            ArgumentBuilderType::Argument(ref argument) => argument,
            _ => panic!("CommandNode::argument() called on non-argument node"),
        }
    }

    pub fn get_relevant_nodes(&self, input: &mut StringReader) -> Vec<Arc<RwLock<CommandNode<S>>>> {
        let literals = &self.literals;

        if literals.is_empty() {
            self.arguments.values().cloned().collect()
        } else {
            let cursor = input.cursor();
            while input.can_read() && input.peek() != ' ' {
                input.skip();
            }
            let text: String = input
                .string()
                .chars()
                .skip(cursor)
                .take(input.cursor() - cursor)
                .collect();
            input.cursor = cursor;
            let literal = literals.get(&text);
            if let Some(literal) = literal {
                vec![literal.clone()]
            } else {
                self.arguments.values().cloned().collect()
            }
        }
    }

    pub fn can_use(&self, source: &S) -> bool {
        (self.requirement)(source)
    }

    pub fn add_child(&mut self, node: &Arc<RwLock<CommandNode<S>>>) {
        let child = self.children.get(node.read().name());
        if let Some(child) = child {
            // We've found something to merge onto
            if let Some(command) = &node.read().command {
                child.write().command = Some(command.clone());
            }
            for grandchild in node.read().children.values() {
                child.write().add_child(grandchild);
            }
        } else {
            self.children
                .insert(node.read().name().to_string(), node.clone());
            match &node.read().value {
                ArgumentBuilderType::Literal(literal) => {
                    self.literals.insert(literal.value.clone(), node.clone());
                }
                ArgumentBuilderType::Argument(argument) => {
                    self.arguments.insert(argument.name.clone(), node.clone());
                }
            }
        }
    }

    pub fn name(&self) -> &str {
        match &self.value {
            ArgumentBuilderType::Argument(argument) => &argument.name,
            ArgumentBuilderType::Literal(literal) => &literal.value,
        }
    }

    pub fn usage_text(&self) -> String {
        match &self.value {
            ArgumentBuilderType::Argument(argument) => format!("<{}>", argument.name),
            ArgumentBuilderType::Literal(literal) => literal.value.to_owned(),
        }
    }

    pub fn child(&self, name: &str) -> Option<Arc<RwLock<CommandNode<S>>>> {
        self.children.get(name).cloned()
    }

    pub fn parse_with_context(
        &self,
        reader: &mut StringReader,
        context_builder: &mut CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
        match self.value {
            ArgumentBuilderType::Argument(ref argument) => {
                let start = reader.cursor();
                let result = argument.parse(reader)?;
                let parsed = ParsedArgument {
                    range: StringRange::between(start, reader.cursor()),
                    result,
                };

                context_builder.with_argument(&argument.name, parsed.clone());
                context_builder.with_node(Arc::new(RwLock::new(self.clone())), parsed.range);

                Ok(())
            }
            ArgumentBuilderType::Literal(ref literal) => {
                let start = reader.cursor();
                let end = self.parse(reader);

                if let Some(end) = end {
                    context_builder.with_node(
                        Arc::new(RwLock::new(self.clone())),
                        StringRange::between(start, end),
                    );
                    return Ok(());
                }

                Err(BuiltInExceptions::LiteralIncorrect {
                    expected: literal.value.clone(),
                }
                .create_with_context(reader))
            }
        }
    }

    fn parse(&self, reader: &mut StringReader) -> Option<usize> {
        match self.value {
            ArgumentBuilderType::Argument(_) => {
                panic!("Can't parse argument.")
            }
            ArgumentBuilderType::Literal(ref literal) => {
                let start = reader.cursor();
                if reader.can_read_length(literal.value.len()) {
                    let end = start + literal.value.len();
                    if reader
                        .string()
                        .get(start..end)
                        .expect("Couldn't slice reader correctly?")
                        == literal.value
                    {
                        reader.cursor = end;
                        if !reader.can_read() || reader.peek() == ' ' {
                            return Some(end);
                        } else {
                            reader.cursor = start;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn list_suggestions(
        &self,
        // context is here because that's how it is in mojang's brigadier, but we haven't
        // implemented custom suggestions yet so this is unused rn
        _context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Suggestions {
        match &self.value {
            ArgumentBuilderType::Literal(literal) => {
                if literal
                    .value
                    .to_lowercase()
                    .starts_with(builder.remaining_lowercase())
                {
                    builder.suggest(&literal.value).build()
                } else {
                    Suggestions::default()
                }
            }
            ArgumentBuilderType::Argument(argument) => argument.list_suggestions(builder),
        }
    }
}

impl<S> Debug for CommandNode<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandNode")
            .field("value", &self.value)
            .field("children", &self.children)
            .field("command", &self.command.is_some())
            // .field("requirement", &self.requirement)
            .field("redirect", &self.redirect)
            .field("forks", &self.forks)
            // .field("modifier", &self.modifier)
            .finish()
    }
}

impl<S> Default for CommandNode<S> {
    fn default() -> Self {
        Self {
            value: ArgumentBuilderType::Literal(Literal::default()),

            children: BTreeMap::new(),
            literals: HashMap::new(),
            arguments: HashMap::new(),

            command: None,
            requirement: Arc::new(|_| true),
            redirect: None,
            forks: false,
            modifier: None,
        }
    }
}

impl<S> Hash for CommandNode<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // hash the children
        for (k, v) in &self.children {
            k.hash(state);
            v.read().hash(state);
        }
        // i hope this works because if doesn't then that'll be a problem
        ptr::hash(&self.command, state);
    }
}

impl<S> PartialEq for CommandNode<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.children.len() != other.children.len() {
            return false;
        }
        for (k, v) in &self.children {
            let other_child = other.children.get(k).unwrap();
            if !Arc::ptr_eq(v, other_child) {
                return false;
            }
        }

        if let Some(selfexecutes) = &self.command {
            // idk how to do this better since we can't compare `dyn Fn`s
            if let Some(otherexecutes) = &other.command {
                #[allow(ambiguous_wide_pointer_comparisons)]
                if !Arc::ptr_eq(selfexecutes, otherexecutes) {
                    return false;
                }
            } else {
                return false;
            }
        } else if other.command.is_some() {
            return false;
        }
        true
    }
}
impl<S> Eq for CommandNode<S> {}
