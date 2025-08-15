use proc_macro2::TokenStream;
use quote::quote;

use crate::inputs::{RootNode, RunInput, StateInput};

pub fn macro_draw_impl(input: RootNode) -> TokenStream {
    quote! { "alo" }
}

pub fn macro_run_impl(input: RunInput) -> TokenStream {
    let RunInput { compname, mode } = input;

    quote! {}
}

pub fn macro_state_impl(input: StateInput) -> TokenStream {
    quote! {}
}
