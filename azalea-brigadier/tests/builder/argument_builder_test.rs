use std::rc::Rc;

use azalea_brigadier::{builder::argument_builder::ArgumentBuilder, prelude::*};

#[test]
fn test_arguments() {
    let builder: ArgumentBuilder<()> = literal("foo");

    let argument: ArgumentBuilder<()> = argument("bar", integer());
    let builder = builder.then(argument.clone());
    assert_eq!(builder.arguments().children.len(), 1);
    let built_argument = Rc::new(argument.build());
    assert!(
        builder
            .arguments()
            .children
            .values()
            .any(|e| *e.read() == *built_argument)
    );
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

//     private static class TestableArgumentBuilder<S> extends
// ArgumentBuilder<S, TestableArgumentBuilder<S>> {         @Override
//         protected TestableArgumentBuilder<S> getThis() {
//             return this;
//         }

//         @Override
//         public CommandNode<S> build() {
//             return null;
//         }
//     }
// }
