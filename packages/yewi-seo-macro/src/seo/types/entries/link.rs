use quote::quote;
use syn::{LitStr, Token};
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

    while !input.is_empty() {
      let lookahead = input.lookahead1();

      if lookahead.peek(kw::author) {
        input.parse::<kw::author>()?;
        input.parse::<Token![=]>()?;
        author = Some(input.parse()?);
      } else if lookahead.peek(kw::manifest) {
        input.parse::<kw::manifest>()?;
        input.parse::<Token![=]>()?;
        manifest = Some(input.parse()?);
      } else if lookahead.peek(kw::canonical) {
        input.parse::<kw::canonical>()?;
        input.parse::<Token![=]>()?;
        canonical = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }
      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
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

mod kw {
  use syn::custom_keyword;

  custom_keyword!(author);
  custom_keyword!(manifest);
  custom_keyword!(canonical);
}