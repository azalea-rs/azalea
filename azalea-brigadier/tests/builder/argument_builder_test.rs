use std::rc::Rc;

use crate::{
    arguments::integer_argument_type::integer,
    builder::{literal_argument_builder::literal, required_argument_builder::argument},
};

use super::ArgumentBuilder;

// public class ArgumentBuilderTest {
//     private TestableArgumentBuilder<Object> builder;

//     @Before
//     public void setUp() throws Exception {
//         builder = new TestableArgumentBuilder<>();
//     }

//     @Test
//     public void testArguments() throws Exception {
//         final RequiredArgumentBuilder<Object, ?> argument = argument("bar", integer());

//         builder.then(argument);

//         assertThat(builder.getArguments(), hasSize(1));
//         assertThat(builder.getArguments(), hasItem((CommandNode<Object>) argument.build()));
//     }

#[test]
fn test_arguments() {
    let mut builder: ArgumentBuilder<()> = literal("foo");

    let argument: ArgumentBuilder<()> = argument("bar", integer());
    builder.then(argument.clone());
    assert_eq!(builder.arguments.children.len(), 1);
    let built_argument = Rc::new(argument.build());
    assert!(builder
        .arguments
        .children
        .values()
        .any(|e| *e.borrow() == *built_argument));
}

//     @Test
//     public void testRedirect() throws Exception {
//         final CommandNode<Object> target = mock(CommandNode.class);
//         builder.redirect(target);
//         assertThat(builder.getRedirect(), is(target));
//     }

//     @Test(expected = IllegalStateException.class)
//     public void testRedirect_withChild() throws Exception {
//         final CommandNode<Object> target = mock(CommandNode.class);
//         builder.then(literal("foo"));
//         builder.redirect(target);
//     }

//     @Test(expected = IllegalStateException.class)
//     public void testThen_withRedirect() throws Exception {
//         final CommandNode<Object> target = mock(CommandNode.class);
//         builder.redirect(target);
//         builder.then(literal("foo"));
//     }

//     private static class TestableArgumentBuilder<S> extends ArgumentBuilder<S, TestableArgumentBuilder<S>> {
//         @Override
//         protected TestableArgumentBuilder<S> getThis() {
//             return this;
//         }

//         @Override
//         public CommandNode<S> build() {
//             return null;
//         }
//     }
// }
