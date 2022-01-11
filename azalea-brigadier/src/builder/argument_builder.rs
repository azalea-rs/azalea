use crate::{
    command::Command,
    redirect_modifier::RedirectModifier,
    single_redirect_modifier::SingleRedirectModifier,
    tree::{command_node::CommandNode, root_command_node::RootCommandNode},
};

pub struct BaseArgumentBuilder<'a, S, T>
where
    S: Sized,
    T: Sized,
{
    arguments: RootCommandNode<'a, S, T>,
    command: Option<&'a dyn Command<S, T>>,
    requirement: &'a dyn Fn(&S) -> bool,
    target: Option<&'a dyn CommandNode<S, T>>,
    modifier: Option<&'a dyn RedirectModifier<S, T>>,
    forks: bool,
}

pub trait ArgumentBuilder<S, T> {
    fn build(self) -> dyn CommandNode<S, T>;
}

impl<S, T> BaseArgumentBuilder<'_, S, T> {
    pub fn then(&mut self, command: dyn CommandNode<S, T>) -> Result<&mut T, String> {
        if self.target.is_some() {
            return Err("Cannot add children to a redirected node".to_string());
        }
        self.command = command;
        Ok(self)
    }

    pub fn arguments(&self) -> &Vec<&dyn CommandNode<S, T>> {
        &self.arguments.get_children()
    }

    pub fn executes(&mut self, command: dyn Command<S, T>) -> &mut T {
        self.command = command;
        self
    }

    pub fn command(&self) -> dyn Command<S, T> {
        self.command
    }

    pub fn requires(&mut self, requirement: &dyn Fn(&S) -> bool) -> &mut T {
        self.requirement = requirement;
        self
    }

    pub fn requirement(&self) -> dyn Fn(&S) -> bool {
        self.requirement
    }

    pub fn redirect(&mut self, target: &dyn CommandNode<S, T>) -> &mut T {
        self.forward(target, None, false)
    }

    pub fn redirect_modifier(
        &mut self,
        target: &dyn CommandNode<S, T>,
        modifier: &dyn SingleRedirectModifier<S, T>,
    ) -> &mut T {
        // forward(target, modifier == null ? null : o -> Collections.singleton(modifier.apply(o)), false);
        self.forward(target, modifier.map(|m| |o| vec![m.apply(o)]), false)
    }

    pub fn fork(
        &mut self,
        target: &dyn CommandNode<S, T>,
        modifier: &dyn RedirectModifier<S, T>,
    ) -> &mut T {
        self.forward(target, Some(modifier), true)
    }

    pub fn forward(
        &mut self,
        target: &dyn CommandNode<S, T>,
        modifier: Option<&dyn RedirectModifier<S, T>>,
        fork: bool,
    ) -> Result<&mut T, String> {
        if !self.arguments.get_children().is_empty() {
            return Err("Cannot forward a node with children".to_string());
        }
        self.target = Some(target);
        self.modifier = modifier;
        self.forks = fork;
        Ok(self)
    }

    pub fn get_redirect(&self) -> Option<&dyn CommandNode<S, T>> {
        self.target.as_ref()
    }

    pub fn get_redirect_modifier(&self) -> Option<&dyn RedirectModifier<S, T>> {
        self.modifier.as_ref()
    }

    pub fn is_fork(&self) -> bool {
        self.forks
    }
}
