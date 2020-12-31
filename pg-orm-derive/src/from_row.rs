use crate::{DeriveInputExt, Errors, FieldExt};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitInt};

pub(crate) fn generate(input: &DeriveInput) -> TokenStream {
    try_token_stream!(impl_from_row(input))
}

fn impl_from_row(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let fields = input.fields()?;
    let ident = &input.ident;
    let mut errors = Vec::new();

    let fields = fields
        .iter()
        .filter_map(|f| f.ident().map_err(|e| errors.push(e)).ok())
        .enumerate()
        .map(|(i, m)| {
            let i = LitInt::new(&i.to_string(), m.span());
            quote! { #m: row.get(#i) }
        });

    let fields = quote! { #(#fields,)* };

    errors.result()?;

    Ok(quote! {
        impl pg_orm::FromRow for #ident {
            fn from_row(row: tokio_postgres::Row) -> Self {
                Self {
                    #fields
                }
            }
        }
    })
}
