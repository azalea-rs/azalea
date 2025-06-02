use std::rc::Rc;

use crate::context::CommandContext;

pub trait ResultConsumer<S> {
    fn on_command_complete(&self, context: Rc<CommandContext<S>>, success: bool, result: i32);
}

pub struct DefaultResultConsumer;
impl<S> ResultConsumer<S> for DefaultResultConsumer {
    fn on_command_complete(&self, _context: Rc<CommandContext<S>>, _success: bool, _result: i32) {}
}
