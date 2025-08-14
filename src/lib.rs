pub use raffaello_macros::draw as draw_impl;

#[macro_export]
macro_rules! draw {
    { $($tt:tt)* } => {
        $crate::draw_impl! { $($tt)* }
    };
    ( $($tt:tt)* ) => {
        compile_error!("Use curly braces: `draw! { ... }`");
    };
    [ $($tt:tt)* ] => {
        compile_error!("Use curly braces: `draw! { ... }`");
    };
}