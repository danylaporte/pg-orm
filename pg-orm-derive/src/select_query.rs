use crate::{DeriveInputExt, Errors, FieldExt, SqlStringExt};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr};

pub(crate) fn generate(input: &DeriveInput) -> TokenStream {
    try_token_stream!(impl_select_query(input))
}

fn impl_select_query(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let fields = input.fields()?;
    let ident = &input.ident;
    let mut errors = Vec::new();
    let mut select = String::new();

    for field in fields {
        let column = continue_token_stream!(field.column(), errors);
        select.add_sep(",").add_field(&column);
    }

    errors.result()?;

    let table = input.table()?;
    let sql = format!("SELECT {} FROM {}", select, table);
    let sql = LitStr::new(&sql, ident.span());
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics pg_orm::SelectQuery for #ident #ty_generics #where_clause {
            fn select_query() -> &'static str {
                #sql
            }
        }
    })
}
