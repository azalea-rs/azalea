#![doc = include_str!("../README.md")]

pub mod arguments;
pub mod builder;
pub mod command_dispatcher;
pub mod context;
pub mod errors;
pub mod modifier;
pub mod parse_results;
pub mod result_consumer;
pub mod string_reader;
pub mod suggestion;
pub mod tree;

pub mod prelude {
    pub use crate::{
        arguments::{
            bool_argument_type::{bool, get_bool},
            double_argument_type::{double, get_double},
            float_argument_type::{float, get_float},
            integer_argument_type::{get_integer, integer},
            long_argument_type::{get_long, long},
            string_argument_type::{get_string, greedy_string, string, word},
        },
        builder::{literal_argument_builder::literal, required_argument_builder::argument},
        command_dispatcher::CommandDispatcher,
        context::CommandContext,
    };
}
