use proc_macro2::TokenStream;
use quote::quote;

use crate::inputs::RootNode;

pub fn macro_draw_impl(input: RootNode) -> TokenStream {
    quote! { "alo" }
}