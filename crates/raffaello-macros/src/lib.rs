mod impls;
mod inputs;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::inputs::{RootNode, RunInput, StateInput};

#[proc_macro]
pub fn draw(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as RootNode);
    impls::macro_draw_impl(input).into()
}

#[proc_macro]
pub fn run(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as RunInput);
    impls::macro_run_impl(input).into()
}

#[proc_macro]
pub fn state(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as StateInput);
    impls::macro_state_impl(input).into()
}