//! The futures-rs procedural macro implementations.

#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_assignments, unused_variables)
    )
))]

// Since https://github.com/rust-lang/cargo/pull/7700 `proc_macro` is part of the prelude for
// proc-macro crates, but to support older compilers we still need this explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;

mod executor;
mod join;
mod select;
mod stream_select;

/// The `join!` macro.
#[cfg_attr(fn_like_proc_macro, proc_macro)]
#[cfg_attr(not(fn_like_proc_macro), proc_macro_hack::proc_macro_hack)]
pub fn join_internal(input: TokenStream) -> TokenStream {
    crate::join::join(input)
}

/// The `try_join!` macro.
#[cfg_attr(fn_like_proc_macro, proc_macro)]
#[cfg_attr(not(fn_like_proc_macro), proc_macro_hack::proc_macro_hack)]
pub fn try_join_internal(input: TokenStream) -> TokenStream {
    crate::join::try_join(input)
}

/// The `select!` macro.
#[cfg_attr(fn_like_proc_macro, proc_macro)]
#[cfg_attr(not(fn_like_proc_macro), proc_macro_hack::proc_macro_hack)]
pub fn select_internal(input: TokenStream) -> TokenStream {
    crate::select::select(input)
}

/// The `select_biased!` macro.
#[cfg_attr(fn_like_proc_macro, proc_macro)]
#[cfg_attr(not(fn_like_proc_macro), proc_macro_hack::proc_macro_hack)]
pub fn select_biased_internal(input: TokenStream) -> TokenStream {
    crate::select::select_biased(input)
}

// TODO: Change this to doc comment once rustdoc bug fixed.
// The `test` attribute.
#[proc_macro_attribute]
pub fn test_internal(input: TokenStream, item: TokenStream) -> TokenStream {
    crate::executor::test(input, item)
}

/// The `stream_select!` macro.
#[cfg_attr(fn_like_proc_macro, proc_macro)]
#[cfg_attr(not(fn_like_proc_macro), proc_macro_hack::proc_macro_hack)]
pub fn stream_select_internal(input: TokenStream) -> TokenStream {
    crate::stream_select::stream_select(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
