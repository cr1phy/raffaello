mod impls;
mod inputs;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::inputs::RootNode;

#[proc_macro]
pub fn draw(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as RootNode);
    impls::macro_draw_impl(input).into()
}