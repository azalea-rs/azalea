use crate::{context::CommandContext, modifier::RedirectModifier, tree::CommandNode};

use super::{literal_argument_builder::Literal, required_argument_builder::Argument};
use std::{any::Any, cell::RefCell, collections::BTreeMap, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub enum ArgumentBuilderType {
    Literal(Literal),
    Argument(Argument),
}

/// A node that hasn't yet been built.
#[derive(Clone)]
pub struct ArgumentBuilder<S: Any + Clone> {
    value: ArgumentBuilderType,

    children: BTreeMap<String, Rc<RefCell<CommandNode<S>>>>,
    literals: BTreeMap<String, Rc<RefCell<CommandNode<S>>>>,
    arguments: BTreeMap<String, Rc<RefCell<CommandNode<S>>>>,

    executes: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
    requirement: Rc<dyn Fn(Rc<S>) -> bool>,
    forks: bool,
    modifier: Option<Rc<dyn RedirectModifier<S>>>,
}

// todo: maybe remake this to be based on a CommandNode like vanilla does?

/// A node that isn't yet built.
impl<S: Any + Clone> ArgumentBuilder<S> {
    pub fn new(value: ArgumentBuilderType) -> Self {
        Self {
            value,
            children: BTreeMap::new(),
            literals: BTreeMap::new(),
            arguments: BTreeMap::new(),
            executes: None,
            requirement: Rc::new(|_| true),
            forks: false,
            modifier: None,
        }
    }

    pub fn then(&mut self, node: ArgumentBuilder<S>) -> &mut Self {
        let built_node = node.build();
        let name = built_node.name();
        let node_reference = Rc::new(RefCell::new(built_node.clone()));
        self.children
            .insert(name.to_string(), node_reference.clone());
        match &built_node.value {
            ArgumentBuilderType::Literal(literal) => {
                self.literals.insert(name.to_string(), node_reference);
            }
            ArgumentBuilderType::Argument(argument) => {
                self.arguments.insert(name.to_string(), node_reference);
            }
        }
        self
    }

    pub fn executes<F>(&mut self, f: F) -> Self
    where
        F: Fn(&CommandContext<S>) -> i32 + 'static,
    {
        self.executes = Some(Rc::new(f));
        self.clone()
    }

    pub fn build(self) -> CommandNode<S> {
        CommandNode {
            value: self.value,

            children: self.children,
            literals: self.literals,
            arguments: self.arguments,

            command: self.executes.clone(),
            requirement: self.requirement.clone(),
            redirect: None,
            forks: self.forks,
            modifier: self.modifier,
        }
    }
}

impl<S: Any + Clone> Debug for ArgumentBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentBuilder")
            .field("value", &self.value)
            .field("children", &self.children)
            .field("literals", &self.literals)
            .field("arguments", &self.arguments)
            .field("executes", &self.executes.is_some())
            // .field("requirement", &self.requirement)
            .field("forks", &self.forks)
            // .field("modifier", &self.modifier)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        parsers::integer,
    };

    use super::ArgumentBuilder;

    // public class ArgumentBuilderTest {
    //     private TestableArgumentBuilder<Object> builder;

    //     @Before
    //     public void setUp() throws Exception {
    //         builder = new TestableArgumentBuilder<>();
    //     }

    //     @Test
    //     public void testArguments() throws Exception {
    //         final RequiredArgumentBuilder<Object, ?> argument = argument("bar", integer());

    //         builder.then(argument);

    //         assertThat(builder.getArguments(), hasSize(1));
    //         assertThat(builder.getArguments(), hasItem((CommandNode<Object>) argument.build()));
    //     }

    #[test]
    fn test_arguments() {
        let mut builder: ArgumentBuilder<()> = literal("foo");

        let argument: ArgumentBuilder<()> = argument("bar", integer());
        builder.then(argument.clone());
        assert_eq!(builder.children.len(), 1);
        let built_argument = Rc::new(argument.build());
        assert!(builder
            .children
            .values()
            .any(|e| *e.borrow() == *built_argument));
    }

    //     @Test
    //     public void testRedirect() throws Exception {
    //         final CommandNode<Object> target = mock(CommandNode.class);
    //         builder.redirect(target);
    //         assertThat(builder.getRedirect(), is(target));
    //     }

    //     @Test(expected = IllegalStateException.class)
    //     public void testRedirect_withChild() throws Exception {
    //         final CommandNode<Object> target = mock(CommandNode.class);
    //         builder.then(literal("foo"));
    //         builder.redirect(target);
    //     }

    //     @Test(expected = IllegalStateException.class)
    //     public void testThen_withRedirect() throws Exception {
    //         final CommandNode<Object> target = mock(CommandNode.class);
    //         builder.redirect(target);
    //         builder.then(literal("foo"));
    //     }

    //     private static class TestableArgumentBuilder<S> extends ArgumentBuilder<S, TestableArgumentBuilder<S>> {
    //         @Override
    //         protected TestableArgumentBuilder<S> getThis() {
    //             return this;
    //         }

    //         @Override
    //         public CommandNode<S> build() {
    //             return null;
    //         }
    //     }
    // }
}
