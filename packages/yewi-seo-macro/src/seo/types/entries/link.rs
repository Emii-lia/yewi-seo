use proc_macro2::Ident;
use quote::quote;
use syn::{Error, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::seo::types::option_litstr_tokens;

pub struct LinkEntry {
  pub author: Option<LitStr>,
  pub manifest: Option<LitStr>,
  pub canonical: Option<LitStr>,
}

impl Parse for LinkEntry {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut author = None;
    let mut manifest = None;
    let mut canonical = None;

    let entries = input.parse_terminated(|input| -> syn::Result<(Ident, LitStr)> {
      let key: Ident = input.parse()?;
      input.parse::<Token![=]>()?;
      let value: LitStr = input.parse()?;
      Ok((key, value))
    }, Token![,], )?;

    for (key, value) in entries {
      match key.to_string().as_str() {
        "author" => author = Some(value),
        "manifest" => manifest = Some(value),
        "canonical" => canonical = Some(value),
        rest => return Err(Error::new(key.span(), format!("Invalid key: {}", rest))),
      }
    }

    Ok(Self {
      author,
      manifest,
      canonical,
    })
  }
}

impl LinkEntry {
  pub fn build_link_tokens(&self) -> proc_macro2::TokenStream {
    let author = option_litstr_tokens(self.author.as_ref());
    let manifest = option_litstr_tokens(self.manifest.as_ref());
    let canonical = option_litstr_tokens(self.canonical.as_ref());

    quote! {
        ::yewi_seo::apply_seo_link(::yewi_seo::LinkProps {
            author: #author,
            manifest: #manifest,
            canonical: #canonical,
        })
    }
  }

}