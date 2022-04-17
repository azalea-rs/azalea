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
    arguments: CommandNode<S>,

    command: Option<Rc<dyn Fn(&CommandContext<S>) -> i32>>,
    requirement: Rc<dyn Fn(Rc<S>) -> bool>,
    target: Option<Rc<RefCell<CommandNode<S>>>>,

    forks: bool,
    modifier: Option<Rc<dyn RedirectModifier<S>>>,
}

/// A node that isn't yet built.
impl<S: Any + Clone> ArgumentBuilder<S> {
    pub fn new(value: ArgumentBuilderType) -> Self {
        Self {
            arguments: CommandNode {
                value,
                ..Default::default()
            },
            command: None,
            requirement: Rc::new(|_| true),
            forks: false,
            modifier: None,
            target: None,
        }
    }

    pub fn then(&mut self, argument: ArgumentBuilder<S>) -> Self {
        self.arguments
            .add_child(&Rc::new(RefCell::new(argument.build())));
        self.clone()
    }

    pub fn executes<F>(&mut self, f: F) -> Self
    where
        F: Fn(&CommandContext<S>) -> i32 + 'static,
    {
        self.command = Some(Rc::new(f));
        self.clone()
    }

    pub fn requires<F>(&mut self, requirement: F) -> Self
    where
        F: Fn(Rc<S>) -> bool + 'static,
    {
        self.requirement = Rc::new(requirement);
        self.clone()
    }

    pub fn redirect(&mut self, target: Rc<RefCell<CommandNode<S>>>) -> Self {
        self.forward(target, None, false)
    }

    pub fn forward(
        &mut self,
        target: Rc<RefCell<CommandNode<S>>>,
        modifier: Option<Rc<dyn RedirectModifier<S>>>,
        fork: bool,
    ) -> Self {
        if !self.arguments.children.is_empty() {
            panic!("Cannot forward a node with children");
        }
        self.target = Some(target);
        self.modifier = modifier;
        self.forks = fork;
        self.clone()
    }

    pub fn build(self) -> CommandNode<S> {
        let mut result = CommandNode {
            value: self.arguments.value,
            command: self.command.clone(),
            requirement: self.requirement.clone(),
            redirect: self.target.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks,
            ..Default::default()
        };

        for (_, argument) in &self.arguments.children {
            result.add_child(argument);
        }

        result
    }
}

impl<S: Any + Clone> Debug for ArgumentBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentBuilder")
            .field("arguments", &self.arguments)
            // .field("command", &self.command)
            // .field("requirement", &self.requirement)
            .field("target", &self.target)
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
        assert_eq!(builder.arguments.children.len(), 1);
        let built_argument = Rc::new(argument.build());
        assert!(builder
            .arguments
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
