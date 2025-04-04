use std::{fmt::Debug, sync::Arc};

use parking_lot::RwLock;

use super::{literal_argument_builder::Literal, required_argument_builder::Argument};
use crate::{
    context::CommandContext,
    modifier::RedirectModifier,
    tree::{Command, CommandNode},
};

#[derive(Debug, Clone)]
pub enum ArgumentBuilderType {
    Literal(Literal),
    Argument(Argument),
}

/// A node that hasn't yet been built.
#[derive(Clone)]
pub struct ArgumentBuilder<S> {
    arguments: CommandNode<S>,

    command: Command<S>,
    requirement: Arc<dyn Fn(&S) -> bool + Send + Sync>,
    target: Option<Arc<RwLock<CommandNode<S>>>>,

    forks: bool,
    modifier: Option<Arc<RedirectModifier<S>>>,
}

/// A node that isn't yet built.
impl<S> ArgumentBuilder<S> {
    pub fn new(value: ArgumentBuilderType) -> Self {
        Self {
            arguments: CommandNode {
                value,
                ..Default::default()
            },
            command: None,
            requirement: Arc::new(|_| true),
            forks: false,
            modifier: None,
            target: None,
        }
    }

    /// Continue building this node with a child node.
    ///
    /// ```
    /// # use azalea_brigadier::prelude::*;
    /// # let mut subject = CommandDispatcher::<()>::new();
    /// literal("foo").then(
    ///     literal("bar").executes(|ctx: &CommandContext<()>| 42)
    /// )
    /// # ;
    /// ```
    pub fn then(self, argument: ArgumentBuilder<S>) -> Self {
        self.then_built(argument.build())
    }

    /// Add an already built child node to this node.
    ///
    /// You should usually use [`Self::then`] instead.
    pub fn then_built(mut self, argument: CommandNode<S>) -> Self {
        self.arguments.add_child(&Arc::new(RwLock::new(argument)));
        self
    }

    /// Set the command to be executed when this node is reached. If this is not
    /// present on a node, it is not a valid command.
    ///
    /// ```
    /// # use azalea_brigadier::prelude::*;
    /// # let mut subject = CommandDispatcher::<()>::new();
    /// # subject.register(
    /// literal("foo").executes(|ctx: &CommandContext<()>| 42)
    /// # );
    /// ```
    pub fn executes<F>(mut self, f: F) -> Self
    where
        F: Fn(&CommandContext<S>) -> i32 + Send + Sync + 'static,
    {
        self.command = Some(Arc::new(f));
        self
    }

    /// Set the requirement for this node to be considered. If this is not
    /// present on a node, it is considered to always pass.
    ///
    /// ```
    /// # use azalea_brigadier::prelude::*;
    /// # use std::sync::Arc;
    /// # pub struct CommandSource {
    /// #     pub opped: bool,
    /// # }
    /// # let mut subject = CommandDispatcher::<CommandSource>::new();
    /// # subject.register(
    /// literal("foo")
    ///     .requires(|s: &CommandSource| s.opped)
    ///     // ...
    ///     # .executes(|ctx: &CommandContext<CommandSource>| 42)
    /// # );
    pub fn requires<F>(mut self, requirement: F) -> Self
    where
        F: Fn(&S) -> bool + Send + Sync + 'static,
    {
        self.requirement = Arc::new(requirement);
        self
    }

    pub fn redirect(self, target: Arc<RwLock<CommandNode<S>>>) -> Self {
        self.forward(target, None, false)
    }

    pub fn fork(
        self,
        target: Arc<RwLock<CommandNode<S>>>,
        modifier: Arc<RedirectModifier<S>>,
    ) -> Self {
        self.forward(target, Some(modifier), true)
    }

    pub fn forward(
        mut self,
        target: Arc<RwLock<CommandNode<S>>>,
        modifier: Option<Arc<RedirectModifier<S>>>,
        fork: bool,
    ) -> Self {
        if !self.arguments.children.is_empty() {
            panic!("Cannot forward a node with children");
        }
        self.target = Some(target);
        self.modifier = modifier;
        self.forks = fork;
        self
    }

    pub fn arguments(&self) -> &CommandNode<S> {
        &self.arguments
    }

    /// Manually build this node into a [`CommandNode`]. You probably don't need
    /// to do this yourself.
    pub fn build(self) -> CommandNode<S> {
        let mut result = CommandNode {
            value: self.arguments.value,
            command: self.command,
            requirement: self.requirement,
            redirect: self.target,
            modifier: self.modifier,
            forks: self.forks,
            arguments: Default::default(),
            children: Default::default(),
            literals: Default::default(),
        };

        for argument in self.arguments.children.values() {
            result.add_child(argument);
        }

        result
    }
}

impl<S> Debug for ArgumentBuilder<S> {
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
