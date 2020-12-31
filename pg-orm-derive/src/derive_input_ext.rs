use super::AttrsExt;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, LitStr};

pub(crate) trait DeriveInputExt {
    fn input(&self) -> &DeriveInput;

    fn fields(&self) -> Result<&Fields, TokenStream> {
        let input = self.input();
        match &input.data {
            Data::Struct(s) => Ok(&s.fields),
            _ => Err(Error::new(input.span(), "Only struct are supported.").to_compile_error()),
        }
    }

    fn table(&self) -> Result<String, TokenStream> {
        self.table_lit().map(|l| l.value())
    }

    fn table_lit(&self) -> Result<LitStr, TokenStream> {
        let input = self.input();

        let table = input.attrs.parse_attr::<LitStr>("table")?.ok_or_else(|| {
            Error::new(input.span(), "table attribute expected").to_compile_error()
        })?;

        //check_table_format(&table).map_err(|e| e.to_compile_error())?;
        Ok(table)
    }
}

impl DeriveInputExt for DeriveInput {
    fn input(&self) -> &DeriveInput {
        self
    }
}
