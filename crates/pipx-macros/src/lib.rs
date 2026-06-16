use proc_macro::TokenStream;

use quote::quote;
use syn::{ItemStruct, Path, Token, parse_macro_input, punctuated::Punctuated};

/// Implements the [`Pipe`] trait for a struct.
///
/// The macro expects a passable type and optionally an error type. When the
/// error type is omitted, [`PipelineError`] is used by default.
///
/// **Parameters**
/// - `PassableType` - The value type processed by the pipe.
/// - `ErrorType` - The error type returned by the pipe.
///
/// **Usage**
/// ```ignore
/// use pipx::{pipe, Next, PipelineResult};
///
/// #[pipe(String)]
/// struct Prefix;
///
/// impl Prefix {
///     fn handle(
///         &self,
///         passable: String,
///         next: Next<'_, String>,
///     ) -> PipelineResult<String> {
///         next.handle(format!("[app] {passable}"))
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn pipe(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;

    let args = parse_macro_input!(args with Punctuated::<Path, Token![,]>::parse_terminated);

    if args.is_empty() || args.len() > 2 {
        return syn::Error::new_spanned(
            &input_struct,
            "expected #[pipe(PassableType)] or #[pipe(PassableType, ErrorType)]",
        )
        .to_compile_error()
        .into();
    }

    let passable_ty = &args[0];

    let error_ty = if args.len() == 2 {
        let error_ty = &args[1];
        quote! { #error_ty }
    } else {
        quote! { pipx::PipelineError }
    };

    let expanded = quote! {
        #input_struct

        impl pipx::Pipe<#passable_ty, #error_ty> for #name {
            fn handle(
                &self,
                passable: #passable_ty,
                next: pipx::Next<'_, #passable_ty, #error_ty>,
            ) -> pipx::PipelineResult<#passable_ty, #error_ty> {
                self.handle(passable, next)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Implements the [`AsyncPipe`] trait for a struct.
///
/// The macro expects a passable type and optionally an error type. When the
/// error type is omitted, [`PipelineError`] is used by default.
///
/// **Parameters**
/// - `PassableType` - The value type processed by the asynchronous pipe.
/// - `ErrorType` - The error type returned by the asynchronous pipe.
///
/// **Usage**
/// ```ignore
/// use async_trait::async_trait;
/// use pipx::{async_pipe, AsyncNext, PipelineResult};
///
/// #[async_pipe(String)]
/// struct Prefix;
///
/// #[async_trait]
/// impl Prefix {
///     async fn handle(
///         &self,
///         passable: String,
///         next: AsyncNext<'_, String>,
///     ) -> PipelineResult<String> {
///         next.handle(format!("[app] {passable}")).await
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn async_pipe(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;

    let args = parse_macro_input!(args with Punctuated::<Path, Token![,]>::parse_terminated);

    if args.is_empty() || args.len() > 2 {
        return syn::Error::new_spanned(
            &input_struct,
            "expected #[async_pipe(PassableType)] or #[async_pipe(PassableType, ErrorType)]",
        )
        .to_compile_error()
        .into();
    }

    let passable_ty = &args[0];

    let error_ty = if args.len() == 2 {
        let error_ty = &args[1];
        quote! { #error_ty }
    } else {
        quote! { pipx::PipelineError }
    };

    let expanded = quote! {
        #input_struct

        #[async_trait::async_trait]
        impl pipx::AsyncPipe<#passable_ty, #error_ty> for #name {
            async fn handle(
                &self,
                passable: #passable_ty,
                next: pipx::AsyncNext<'_, #passable_ty, #error_ty>,
            ) -> pipx::PipelineResult<#passable_ty, #error_ty> {
                self.handle(passable, next).await
            }
        }
    };

    TokenStream::from(expanded)
}
