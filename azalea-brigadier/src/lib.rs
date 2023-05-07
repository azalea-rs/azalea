#![doc = include_str!("../README.md")]

pub mod arguments;
pub mod builder;
pub mod command_dispatcher;
pub mod context;
pub mod exceptions;
pub mod modifier;
pub mod parse_results;
pub mod string_reader;
pub mod suggestion;
pub mod tree;

pub mod prelude {
    pub use crate::{
        arguments::{
            double_argument_type::double, float_argument_type::float,
            integer_argument_type::integer, long_argument_type::long, string_argument_type::string,
        },
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        command_dispatcher::CommandDispatcher,
        context::CommandContext,
    };
}
