use std::{rc::Rc, sync::Arc};

use super::CommandContext;
use crate::{
    errors::{CommandResultTrait, CommandSyntaxError},
    result_consumer::ResultConsumer,
};

pub struct ContextChain<S, R> {
    modifiers: Vec<Rc<CommandContext<S, R>>>,
    executable: Rc<CommandContext<S, R>>,
    next_stage_cache: Option<Rc<ContextChain<S, R>>>,
}

impl<S, R: CommandResultTrait> ContextChain<S, R> {
    pub fn new(
        modifiers: Vec<Rc<CommandContext<S, R>>>,
        executable: Rc<CommandContext<S, R>>,
    ) -> Self {
        if executable.command.is_none() {
            panic!("Last command in chain must be executable");
        }
        Self {
            modifiers,
            executable,
            next_stage_cache: None,
        }
    }

    pub fn try_flatten(root_context: Rc<CommandContext<S, R>>) -> Option<Self> {
        let mut modifiers = Vec::new();
        let mut current = root_context;
        loop {
            let child = current.child.clone();
            let Some(child) = child else {
                // Last entry must be executable command
                current.command.as_ref()?;

                return Some(ContextChain::new(modifiers, current));
            };

            modifiers.push(current);
            current = child;
        }
    }

    pub fn run_modifier(
        modifier: Rc<CommandContext<S, R>>,
        source: Arc<S>,
        result_consumer: &dyn ResultConsumer<S, R>,
        forked_mode: bool,
    ) -> Result<Vec<Arc<S>>, CommandSyntaxError> {
        let source_modifier = modifier.redirect_modifier();
        let Some(source_modifier) = source_modifier else {
            return Ok(vec![source]);
        };

        let context_to_use = Rc::new(modifier.copy_for(source));
        let err = match (source_modifier)(&context_to_use) {
            Ok(res) => return Ok(res),
            Err(e) => e,
        };

        result_consumer.on_command_complete(context_to_use, false, 0);
        if forked_mode {
            return Ok(vec![]);
        }
        Err(err)
    }

    pub fn run_executable(
        &self,
        executable: Rc<CommandContext<S, R>>,
        source: Arc<S>,
        result_consumer: &dyn ResultConsumer<S, R>,
        forked_mode: bool,
    ) -> Result<R, CommandSyntaxError> {
        let context_to_use = Rc::new(executable.copy_for(source));
        let Some(command) = &executable.command else {
            unimplemented!();
        };

        let res = (command)(&context_to_use);
        let err = match res {
            Ok(res) => {
                let Some(res) = res.as_i32() else {
                    // these are treated as exceptions, so they can bubble up without doing anything
                    // else
                    return Ok(res);
                };
                result_consumer.on_command_complete(context_to_use, true, res);
                return if forked_mode {
                    Ok(R::new(1))
                } else {
                    Ok(R::new(res))
                };
            }
            Err(err) => err,
        };

        result_consumer.on_command_complete(context_to_use, false, 0);
        if forked_mode { Ok(R::new(0)) } else { Err(err) }
    }

    pub fn execute_all(
        &self,
        source: Arc<S>,
        result_consumer: &dyn ResultConsumer<S, R>,
    ) -> Result<R, CommandSyntaxError> {
        if self.modifiers.is_empty() {
            return self.run_executable(self.executable.clone(), source, result_consumer, false);
        }

        let mut forked_mode = false;
        let mut current_sources = vec![source];

        for modifier in &self.modifiers {
            forked_mode |= modifier.is_forked();

            let mut next_sources = Vec::new();
            for source_to_run in current_sources {
                let res = Self::run_modifier(
                    modifier.clone(),
                    source_to_run.clone(),
                    result_consumer,
                    forked_mode,
                )?;
                next_sources.extend(res);
            }
            if next_sources.is_empty() {
                return Ok(R::new(0));
            }
            current_sources = next_sources;
        }

        let mut summed = 0;
        for execution_source in current_sources {
            let res = self.run_executable(
                self.executable.clone(),
                execution_source,
                result_consumer,
                forked_mode,
            )?;
            match res.as_i32() {
                Some(res) => summed += res,
                None => return Ok(res),
            }
        }

        Ok(R::new(summed))
    }

    pub fn stage(&self) -> Stage {
        if self.modifiers.is_empty() {
            Stage::Execute
        } else {
            Stage::Modify
        }
    }

    pub fn top_context(&self) -> Rc<CommandContext<S, R>> {
        self.modifiers
            .first()
            .cloned()
            .unwrap_or_else(|| self.executable.clone())
    }

    pub fn next_stage(&mut self) -> Option<Rc<ContextChain<S, R>>> {
        let modifier_count = self.modifiers.len();
        if modifier_count == 0 {
            return None;
        }

        if self.next_stage_cache.is_none() {
            self.next_stage_cache = Some(Rc::new(ContextChain::new(
                self.modifiers[1..].to_vec(),
                self.executable.clone(),
            )));
        }

        self.next_stage_cache.clone()
    }
}

pub enum Stage {
    Modify,
    Execute,
}
