use crate::{
    arguments::argument_type::ArgumentType,
    command::Command,
    redirect_modifier::RedirectModifier,
    single_redirect_modifier::SingleRedirectModifier,
    tree::{
        command_node::{BaseCommandNode, CommandNode},
        root_command_node::RootCommandNode,
    },
};

pub struct BaseArgumentBuilder<'a, S>
where
    S: Sized,
{
    arguments: RootCommandNode<'a, S>,
    command: Option<Box<dyn Command<S>>>,
    requirement: Box<dyn Fn(&S) -> bool>,
    target: Option<Box<dyn CommandNode<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
}

pub trait ArgumentBuilder<S, T>
where
    T: ArgumentBuilder<S, T>,
{
    fn build(self) -> Box<dyn CommandNode<S>>;
}

impl<'a, S> BaseArgumentBuilder<'a, S> {
    pub fn then(
        &mut self,
        command: Box<dyn ArgumentBuilder<S, Self>>,
    ) -> Result<&mut Self, String> {
        if self.target.is_some() {
            return Err("Cannot add children to a redirected node".to_string());
        }
        self.command = Some(command);
        Ok(self)
    }

    pub fn arguments(&self) -> &Vec<&dyn CommandNode<S>> {
        &self.arguments.get_children()
    }

    pub fn executes(&mut self, command: dyn Command<S>) -> &mut Self {
        self.command = command;
        self
    }

    pub fn command(&self) -> dyn Command<S> {
        self.command
    }

    pub fn requires(&mut self, requirement: &dyn Fn(&S) -> bool) -> &mut Self {
        self.requirement = requirement;
        self
    }

    pub fn requirement(&self) -> dyn Fn(&S) -> bool {
        self.requirement
    }

    pub fn redirect(&mut self, target: &dyn CommandNode<S>) -> &mut Self {
        self.forward(target, None, false)
    }

    pub fn redirect_modifier(
        &mut self,
        target: &dyn CommandNode<S>,
        modifier: &dyn SingleRedirectModifier<S>,
    ) -> &mut Self {
        // forward(target, modifier == null ? null : o -> Collections.singleton(modifier.apply(o)), false);
        self.forward(target, modifier.map(|m| |o| vec![m.apply(o)]), false)
    }

    pub fn fork(
        &mut self,
        target: &dyn CommandNode<S>,
        modifier: &dyn RedirectModifier<S>,
    ) -> &mut Self {
        self.forward(target, Some(modifier), true)
    }

    pub fn forward(
        &mut self,
        target: &dyn CommandNode<S>,
        modifier: Option<&dyn RedirectModifier<S>>,
        fork: bool,
    ) -> Result<&mut Self, String> {
        if !self.arguments.get_children().is_empty() {
            return Err("Cannot forward a node with children".to_string());
        }
        self.target = Some(target);
        self.modifier = modifier;
        self.forks = fork;
        Ok(self)
    }

    pub fn get_redirect(&self) -> Option<&dyn CommandNode<S>> {
        self.target.as_ref()
    }

    pub fn get_redirect_modifier(&self) -> Option<&dyn RedirectModifier<S>> {
        self.modifier.as_ref()
    }

    pub fn is_fork(&self) -> bool {
        self.forks
    }

    pub fn build(self) -> BaseCommandNode<'a, S> {
        let result: BaseCommandNode<'a, S> = BaseCommandNode {
            command: self.command,
            requirement: self.requirement,
            redirect: self.target,
            modifier: self.modifier,
            forks: self.forks,

            arguments: Default::default(),
            children: Default::default(),
            literals: Default::default(),
        };

        for argument in self.arguments() {
            result.add_child(argument);
        }

        result
    }
}
