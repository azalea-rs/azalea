mod ambiguity_consumer;
mod arguments;
mod builder;
mod command;
mod command_dispatcher;
mod context;
mod exceptions;
mod immutable_string_reader;
mod literal_message;
mod message;
mod parse_results;
mod redirect_modifier;
mod result_consumer;
mod single_redirect_modifier;
mod string_reader;
mod suggestion;
mod tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
