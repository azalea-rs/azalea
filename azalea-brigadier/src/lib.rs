pub mod builder;
pub mod context;
pub mod dispatcher;
pub mod exceptions;
pub mod message;
pub mod modifier;
pub mod parse_results;
pub mod parsers;
pub mod string_range;
pub mod string_reader;
pub mod tree;

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        dispatcher::CommandDispatcher,
        parsers::{get_integer, integer},
    };

    struct CommandSourceStack {
        player: String,
    }

    #[test]
    fn it_works() {
        let mut dispatcher = CommandDispatcher::<Rc<CommandSourceStack>>::new();

        let source = Rc::new(CommandSourceStack {
            player: "player".to_string(),
        });

        dispatcher.register(
            literal("foo")
                .then(argument("bar", integer()).executes(|c| {
                    println!("Bar is {:?}", get_integer(c, "bar"));
                    2
                }))
                .executes(|c| {
                    println!("Called foo with no arguments");
                    1
                }),
        );

        let parse = dispatcher.parse("foo 123".into(), source.clone());
        assert_eq!(CommandDispatcher::<_>::execute_parsed(parse).unwrap(), 2);
        assert_eq!(dispatcher.execute("foo".into(), source).unwrap(), 1);
    }
}
