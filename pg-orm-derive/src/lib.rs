#[macro_use]
mod macros;

mod attrs_ext;
mod derive_input_ext;
mod errors;
mod field_ext;
mod from_row;
mod load_from_sql;
mod select_query;
mod sql_string_ext;
mod upsert_to_sql;

use attrs_ext::AttrsExt;
use derive_input_ext::DeriveInputExt;
use errors::Errors;
use field_ext::FieldExt;
use proc_macro::TokenStream;
use sql_string_ext::SqlStringExt;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(LoadFromSql, attributes(column, key, table))]
pub fn load_from_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    load_from_sql::generate(&input).into()
}

#[proc_macro_derive(UpsertToSql, attributes(column, key, table))]
pub fn upsert_to_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    upsert_to_sql::generate(&input).into()
}
