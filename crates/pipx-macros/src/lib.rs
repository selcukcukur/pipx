use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Path, Token, parse_macro_input, punctuated::Punctuated};

/// Implements the middleware `Pipe` trait for a struct.
///
/// **Parameters**
/// - The first argument is the passable type.
/// - The second argument is the error type.
///
/// **Usage**
/// ```ignore
/// use pipx::{Next, PipeResult};
/// use pipx::pipe;
///
/// #[pipe(String, pipx::PipelineError)]
/// struct Prefix;
///
/// impl Prefix {
///     fn handle(
///         &self,
///         passable: String,
///         next: Next<'_, String, pipx::PipelineError>,
///     ) -> PipeResult<String> {
///         next.handle(format!("[app] {passable}"))
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn pipe(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_pipe(args, input, PipeKind::Middleware)
}

/// Implements the `TransformPipe` trait for a struct.
///
/// **Parameters**
/// - The first argument is the passable type.
/// - The second argument is the error type.
#[proc_macro_attribute]
pub fn transform_pipe(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_pipe(args, input, PipeKind::Transform)
}

enum PipeKind {
    Middleware,
    Transform,
}

fn expand_pipe(args: TokenStream, input: TokenStream, kind: PipeKind) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;
    let args = parse_macro_input!(args with Punctuated::<Path, Token![,]>::parse_terminated);

    if args.len() != 2 {
        return syn::Error::new_spanned(&input_struct, "expected #[pipe(PassableType, ErrorType)]")
            .to_compile_error()
            .into();
    }

    let passable_ty = &args[0];
    let error_ty = &args[1];

    let expanded = match kind {
        PipeKind::Middleware => quote! {
            #input_struct

            impl pipx::Pipe<#passable_ty, #error_ty> for #name {
                fn handle(
                    &self,
                    passable: #passable_ty,
                    next: pipx::Next<'_, #passable_ty, #error_ty>,
                ) -> pipx::PipeResult<#passable_ty, #error_ty> {
                    self.handle(passable, next)
                }
            }
        },
        PipeKind::Transform => quote! {
            #input_struct

            impl pipx::TransformPipe<#passable_ty, #error_ty> for #name {
                fn handle(
                    &self,
                    passable: #passable_ty,
                ) -> pipx::TransformPipeResult<#passable_ty, #error_ty> {
                    self.handle(passable)
                }
            }
        },
    };

    TokenStream::from(expanded)
}
