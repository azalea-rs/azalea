use crate::{
    builder::{
        argument_builder::ArgumentBuilderType, literal_argument_builder::Literal,
        required_argument_builder::Argument,
    },
    context::{CommandContext, CommandContextBuilder, ParsedArgument, StringRange},
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    modifier::RedirectModifier,
    string_reader::StringReader,
};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, ptr, rc::Rc};

pub type Command<S> = Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>;

/// An ArgumentBuilder that has been built.
#[non_exhaustive]
pub struct CommandNode<S> {
    pub value: ArgumentBuilderType,

    pub children: HashMap<String, Rc<RefCell<CommandNode<S>>>>,
    pub literals: HashMap<String, Rc<RefCell<CommandNode<S>>>>,
    pub arguments: HashMap<String, Rc<RefCell<CommandNode<S>>>>,

    pub command: Command<S>,
    pub requirement: Rc<dyn Fn(Rc<S>) -> bool>,
    pub redirect: Option<Rc<RefCell<CommandNode<S>>>>,
    pub forks: bool,
    pub modifier: Option<Rc<RedirectModifier<S>>>,
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

    pub fn get_relevant_nodes(&self, input: &mut StringReader) -> Vec<Rc<RefCell<CommandNode<S>>>> {
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

    pub fn can_use(&self, source: Rc<S>) -> bool {
        (self.requirement)(source)
    }

    pub fn add_child(&mut self, node: &Rc<RefCell<CommandNode<S>>>) {
        let child = self.children.get(node.borrow().name());
        if let Some(child) = child {
            // We've found something to merge onto
            if let Some(command) = &node.borrow().command {
                child.borrow_mut().command = Some(command.clone());
            }
            for grandchild in node.borrow().children.values() {
                child.borrow_mut().add_child(grandchild);
            }
        } else {
            self.children
                .insert(node.borrow().name().to_string(), node.clone());
            match &node.borrow().value {
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

    pub fn child(&self, name: &str) -> Option<Rc<RefCell<CommandNode<S>>>> {
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
                context_builder.with_node(Rc::new(RefCell::new(self.clone())), parsed.range);

                Ok(())
            }
            ArgumentBuilderType::Literal(ref literal) => {
                let start = reader.cursor();
                let end = self.parse(reader);

                if let Some(end) = end {
                    context_builder.with_node(
                        Rc::new(RefCell::new(self.clone())),
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

            children: HashMap::new(),
            literals: HashMap::new(),
            arguments: HashMap::new(),

            command: None,
            requirement: Rc::new(|_| true),
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
            v.borrow().hash(state);
        }
        // i hope this works because if doesn't then that'll be a problem
        ptr::hash(&self.command, state);
    }
}

impl<S> PartialEq for CommandNode<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.children != other.children {
            return false;
        }
        if let Some(selfexecutes) = &self.command {
            // idk how to do this better since we can't compare `dyn Fn`s
            if let Some(otherexecutes) = &other.command {
                #[allow(clippy::vtable_address_comparisons)]
                if !Rc::ptr_eq(selfexecutes, otherexecutes) {
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
