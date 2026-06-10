use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Pipe)]
pub fn derive_pipe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    // Kullanıcının generic parametrelerini derive içine geçiriyoruz
    let expanded = quote! {
        impl #generics Pipe<TPassable, TError> for #name #generics
        where
            TPassable: Send + Sync + 'static,
            TError: std::fmt::Debug + Send + Sync + 'static,
        {
            fn handle(&self, passable: TPassable) -> Result<TPassable, TError> {
                self.handle(passable)
            }
        }
    };

    TokenStream::from(expanded)
}
