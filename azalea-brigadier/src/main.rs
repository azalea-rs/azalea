use std::rc::Rc;

use rust_command_parser::{
    builder::{literal_argument_builder::literal, required_argument_builder::argument},
    dispatcher::CommandDispatcher,
    parsers::integer,
};

struct CommandSourceStack {
    player: String,
}

pub fn main() {
    let mut dispatcher = CommandDispatcher::<Rc<CommandSourceStack>>::new();

    let source = Rc::new(CommandSourceStack {
        player: "player".to_string(),
    });

    dispatcher.register(
        literal("foo")
            .then(argument("bar", integer()).executes(|c| {
                // println!("Bar is {}", get_integer(c, "bar"));
                2
            }))
            .executes(|c| {
                println!("Called foo with no arguments");
                1
            }),
    );

    let parse = dispatcher.parse("foo 123".to_string().into(), source);
    println!("{:?}", parse);
    println!(
        "{}",
        CommandDispatcher::<Rc<CommandSourceStack>>::execute(parse).unwrap()
    );
}
