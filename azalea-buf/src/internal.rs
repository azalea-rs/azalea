use std::fmt::Debug;

// re-export because the AzBuf macro will sometimes use this
pub use tracing;

pub trait DebugUnknown {
    fn internal_fmt(&self) -> String;
}
impl<T> DebugUnknown for &T {
    fn internal_fmt(&self) -> String {
        "<unknown>".to_owned()
    }
}
pub trait DebugNormal {
    fn internal_fmt(&self) -> String;
}
impl<T: Debug> DebugNormal for T {
    fn internal_fmt(&self) -> String {
        format!("{self:?}")
    }
}
// autoref specialization <3
#[macro_export]
macro_rules! debug_format_any {
    ($e:expr) => {{
        #[allow(unused)]
        use $crate::internal::{DebugNormal, DebugUnknown};
        (&$e).internal_fmt()
    }};
}
pub use debug_format_any;

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug_format_any() {
        struct Undebuggable;

        assert_eq!(debug_format_any!(1), "1");
        assert_eq!(debug_format_any!(Undebuggable), "<unknown>");
    }
}
