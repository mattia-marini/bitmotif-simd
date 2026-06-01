extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprRange, Lit, RangeLimits, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

// Define a struct to hold our parsed macro arguments: iota!(Type, Range)
struct IotaInput {
    ty: Type,
    _comma: syn::Token![,],
    range: ExprRange,
}

impl Parse for IotaInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(IotaInput {
            ty: input.parse()?,
            _comma: input.parse()?,
            range: input.parse()?,
        })
    }
}

// Helper function to recursively evaluate integer literals (including negative numbers)
fn eval_int(expr: &Expr) -> Result<i128, &'static str> {
    match expr {
        Expr::Lit(syn::ExprLit {
            lit: Lit::Int(lit_int),
            ..
        }) => lit_int
            .base10_parse::<i128>()
            .map_err(|_| "Failed to parse integer"),
        Expr::Unary(syn::ExprUnary {
            op: syn::UnOp::Neg(_),
            expr,
            ..
        }) => {
            let val = eval_int(expr)?;
            Ok(-val)
        }
        _ => Err("Expected a literal integer value"),
    }
}

pub fn iota(input: TokenStream) -> TokenStream {
    // Parse the input tokens into our structured IotaInput
    let input = parse_macro_input!(input as IotaInput);

    // Ensure the range has an explicit start bound
    let start_expr = match &input.range.start {
        Some(expr) => expr,
        None => {
            return syn::Error::new_spanned(input.range, "Range must have an explicit start bound")
                .to_compile_error()
                .into();
        }
    };

    // Ensure the range has an explicit end bound
    let end_expr = match &input.range.end {
        Some(expr) => expr,
        None => {
            return syn::Error::new_spanned(input.range, "Range must have an explicit end bound")
                .to_compile_error()
                .into();
        }
    };

    // Evaluate bounds to actual i128 numbers at compile time
    let start = match eval_int(start_expr) {
        Ok(val) => val,
        Err(err) => {
            return syn::Error::new_spanned(start_expr, err)
                .to_compile_error()
                .into();
        }
    };

    let end = match eval_int(end_expr) {
        Ok(val) => val,
        Err(err) => {
            return syn::Error::new_spanned(end_expr, err)
                .to_compile_error()
                .into();
        }
    };

    // Collect the sequence depending on whether it is an exclusive (..) or inclusive (..=) range
    let values: Vec<i128> = match input.range.limits {
        RangeLimits::HalfOpen(_) => (start..end).collect(),
        RangeLimits::Closed(_) => (start..=end).collect(),
    };

    let ty = &input.ty;
    let len = values.len();

    // Generate the final array wrapped in an inline block expression.
    // Specifying the explicit type and length ensures standard literal coercion rules apply.
    let expanded = quote! {
        {
            const __IOTA_ARR: [#ty; #len] = [#(#values),*];
            __IOTA_ARR
        }
    };

    TokenStream::from(expanded)
}
