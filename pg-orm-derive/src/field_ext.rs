use super::attrs_ext::AttrsExt;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Error, Field, Ident, LitStr};

pub(crate) trait FieldExt {
    fn field(&self) -> &Field;

    fn ident(&self) -> Result<&Ident, TokenStream> {
        let f = self.field();
        f.ident
            .as_ref()
            .ok_or_else(|| Error::new(f.span(), "Ident expected.").to_compile_error())
    }

    fn column(&self) -> Result<String, TokenStream> {
        Ok(match self.field().attrs.parse_attr::<LitStr>("column")? {
            Some(c) => c.value(),
            None => self.ident()?.to_string(),
        })
    }

    fn is_key(&self) -> bool {
        self.field().attrs.iter().any(|a| a.path.is_ident("key"))
    }
}

impl FieldExt for Field {
    fn field(&self) -> &Field {
        self
    }
}
