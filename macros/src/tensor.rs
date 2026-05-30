use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, LitInt, Token, parse_macro_input, punctuated::Punctuated};

/// 1. The Multi-Dimension Macro: Handles things like `create_tensor!(3, 4, 5)`
pub fn build_tensor(input: TokenStream) -> TokenStream {
    // Parse a comma-separated list of expressions (e.g., 3, 4, 5)
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let dims = parse_macro_input!(input with parser);

    let mut tokens = quote! { 0usize };

    // To mirror your original macro, we walk backwards from the innermost
    // dimension to the outermost dimension.
    for dim in dims.iter().rev() {
        tokens = quote! { [#tokens; #dim] };
    }

    TokenStream::from(tokens)
}

/// 2. The Uniform Macro: Handles a single integer and builds N-dimensions directly
pub fn build_uniform_tensor(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitInt);

    let n: usize = match lit.base10_parse() {
        Ok(val) => val,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut tokens = quote! { 0usize };

    // Bypasses everything else and builds the type standalone!
    for _ in 0..n {
        tokens = quote! { [#tokens; #lit] };
    }

    TokenStream::from(tokens)
}
