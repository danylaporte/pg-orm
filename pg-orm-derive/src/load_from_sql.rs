use crate::{DeriveInputExt, Errors, FieldExt, SqlStringExt};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, LitInt, LitStr};

pub(crate) fn generate(input: &DeriveInput) -> TokenStream {
    let impl_from_row = crate::from_row::generate(input);
    let impl_load_from_sql = try_token_stream!(impl_load_from_sql(input));
    let impl_select_query = crate::select_query::generate(input);

    quote! {
        #impl_from_row
        #impl_load_from_sql
        #impl_select_query
    }
}

fn impl_load_from_sql(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let ident = &input.ident;
    let keys = input.fields()?.iter().filter(|f| f.is_key()).enumerate();

    let mut errors = Vec::new();
    let mut clauses = String::new();
    let mut params = Vec::new();
    let mut types = Vec::new();

    for (index, field) in keys {
        let column = continue_token_stream!(field.column(), errors);

        clauses.add_sep(" AND ").add(&format!(
            r#"COALESCE(${}, "{}")="{}""#,
            index + 1,
            column,
            column,
        ));

        let param = LitInt::new(&index.to_string(), field.span());
        params.push(quote! { &p.#param as _ });
        types.push(&field.ty);
    }

    errors.result()?;

    if !clauses.is_empty() {
        clauses.insert_str(0, " WHERE ");
    }

    let clauses = LitStr::new(&clauses, ident.span());
    let params = quote! { vec![#(#params,)*] };
    let types = quote! { (#(Option<#types>,)*) };

    Ok(quote! {
        impl pg_orm::LoadFromSql<#types> for #ident {
            fn load_from_sql_params(p: &#types) -> Vec<&(dyn tokio_postgres::types::ToSql + Sync)>  {
                #params
            }

            fn load_from_sql_query() -> String  {
                let mut s = <Self as pg_orm::SelectQuery>::select_query().to_string();
                s.push_str(#clauses);
                s
            }
        }
    })
}
