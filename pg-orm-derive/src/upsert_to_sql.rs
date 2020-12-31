use crate::{DeriveInputExt, Errors, FieldExt, SqlStringExt};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Error, LitStr};

pub(crate) fn generate(input: &DeriveInput) -> TokenStream {
    let impl_upsert_to_sql = try_token_stream!(impl_upsert_to_sql(input));

    quote! {
        #impl_upsert_to_sql
    }
}

fn impl_upsert_to_sql(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let table = input.table()?;
    let fields = input.fields()?;
    let ident = &input.ident;
    let mut errors = Vec::new();

    let mut insert_names = String::new();
    let mut insert_values = String::new();
    let mut keys = String::new();
    let mut params = Vec::new();
    let mut updates = String::new();

    for (index, field) in fields.iter().enumerate() {
        let column = continue_token_stream!(field.column(), errors);
        let field_ident = continue_token_stream!(field.ident(), errors);

        insert_names.add_sep(",").add_field(&column);
        insert_values.add_sep(",").add_param(index + 1);

        if field.is_key() {
            keys.add_sep(",").add_field(&column);
        } else if field_ident != "create_at" && field_ident != "create_by" {
            updates
                .add_sep(",")
                .add_field(&column)
                .add("=")
                .add_param(index + 1);
        }

        params.push(quote! { &self.#field_ident as &(dyn tokio_postgres::types::ToSql + Sync) });
    }

    let params = quote! { vec![#(#params,)*] };

    errors.result()?;

    if keys.is_empty() {
        return Err(Error::new(input.span(), "[key] attribute expected.").to_compile_error());
    }

    if !updates.is_empty() {
        updates.insert_str(0, &format!(" ON CONFLICT ({}) DO UPDATE SET", keys));
    }

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({}){}",
        table, insert_names, insert_values, updates,
    );

    let sql = LitStr::new(&sql, input.span());

    Ok(quote! {
        impl pg_orm::UpsertToSql for #ident {
            fn upsert_params(&self) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)> {
                #params
            }

            fn upsert_query() -> &'static str {
                #sql
            }
        }
    })
}
